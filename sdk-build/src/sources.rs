//! Pinned Mistral source verification. No network access and no Python runtime.
use crate::gates::{Result, fail};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

const OVERLAY_ASSUMPTIONS: [(&str, &str); 3] = [
    (
        "ChatCompletionResponse still requires undeclared data",
        "- data\n",
    ),
    (
        "SharingDelete still requires undeclared level",
        "    SharingDelete:\n",
    ),
    (
        "WorkflowListResponse still carries beta.workflows typo",
        "- beta.workflows\n",
    ),
];

const RETIRED_LOCAL_PATHS: [&str; 4] = [
    "/v1/chat/completions#stream:",
    "/v1/fim/completions#stream:",
    "/v1/audio/speech#stream:",
    "/v1/audio/voices/{voice_id}/sample#wav:",
];

/// Verify the pinned snapshot and all assumptions that justify reviewed overlays.
fn verify_pinned(source: &[u8], expected_sha256: &str) -> Result<String> {
    // Hash the original bytes, never an OpenAPI parser's reserialization.
    let actual = format!("{:x}", Sha256::digest(source));
    if actual != expected_sha256 {
        return fail(format!(
            "pinned OpenAPI SHA-256 mismatch: expected {expected_sha256}, got {actual}"
        ));
    }
    let text = std::str::from_utf8(source)?;
    if !text.starts_with("openapi: 3.1.") {
        return fail("pinned OpenAPI dialect changed");
    }
    for (description, needle) in OVERLAY_ASSUMPTIONS {
        if !text.contains(needle) {
            return fail(format!("review Overlay: assumption changed: {description}"));
        }
    }
    for path in RETIRED_LOCAL_PATHS {
        if text.contains(&format!("  {path}")) {
            return fail(format!(
                "pinned OpenAPI now contains retired local path {}",
                path.trim_end_matches(':')
            ));
        }
    }
    Ok(actual)
}

pub(crate) fn verify_sources(root: &Path, lock: &Value) -> Result<()> {
    let snapshot = root.join("sdk-build/openapi/published.yaml");
    let expected_sha256 = lock["openapi"]["sha256"]
        .as_str()
        .ok_or("OpenAPI SHA-256 pin is missing")?;
    let source = fs::read(snapshot)?;
    let actual = verify_pinned(&source, expected_sha256)?;
    println!("pinned OpenAPI verified: {actual}");

    let candidate_path = root.join("sdk-build/openapi/public-288.yaml");
    let candidate_expected = lock["openapi_candidate"]["sha256"]
        .as_str()
        .ok_or("candidate OpenAPI SHA-256 pin is missing")?;
    let candidate = fs::read(candidate_path)?;
    let candidate_actual = format!("{:x}", Sha256::digest(&candidate));
    if candidate_actual != candidate_expected {
        return fail(format!(
            "candidate OpenAPI SHA-256 mismatch: expected {candidate_expected}, got {candidate_actual}"
        ));
    }
    let candidate_text = std::str::from_utf8(&candidate)?;
    if !candidate_text.starts_with("openapi: 3.1.") {
        return fail("candidate OpenAPI dialect changed");
    }
    println!("staged candidate OpenAPI verified: {candidate_actual}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gates::read_json;

    const VALID: &str = "openapi: 3.1.0\n- data\n    SharingDelete:\n- beta.workflows\n";

    fn hash(source: &[u8]) -> String {
        format!("{:x}", Sha256::digest(source))
    }

    #[test]
    fn accepts_pinned_bytes_and_rejects_any_mutation() {
        let pin = hash(VALID.as_bytes());
        assert_eq!(verify_pinned(VALID.as_bytes(), &pin).unwrap(), pin);
        assert!(verify_pinned(format!("{VALID}# unreviewed\n").as_bytes(), &pin).is_err());
        assert!(verify_pinned(VALID.as_bytes(), "not-a-sha256").is_err());
        assert!(verify_pinned(b"", &hash(b"")).is_err());
    }

    #[test]
    fn rejects_dialect_and_non_utf8_even_with_valid_hash() {
        for bytes in [
            VALID.replace("openapi: 3.1.", "openapi: 3.0.").into_bytes(),
            [VALID.as_bytes(), b"\xff"].concat(),
        ] {
            assert!(verify_pinned(&bytes, &hash(&bytes)).is_err());
        }
    }

    #[test]
    fn fails_closed_on_each_reviewed_overlay_assumption() {
        for (_, needle) in OVERLAY_ASSUMPTIONS {
            let changed = VALID.replace(needle, "");
            assert!(verify_pinned(changed.as_bytes(), &hash(changed.as_bytes())).is_err());
        }
    }

    #[test]
    fn rejects_retired_local_paths_even_with_matching_hash() {
        for path in RETIRED_LOCAL_PATHS {
            let source = format!("{VALID}  {path}\n");
            assert!(verify_pinned(source.as_bytes(), &hash(source.as_bytes())).is_err());
        }
    }

    #[test]
    fn checked_in_openapi_matches_the_pinned_source() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("sdk-build has a repository parent");
        let lock = read_json(&root.join("sdk-build/provenance.lock.json")).unwrap();
        verify_sources(root, &lock).unwrap();
    }
}
