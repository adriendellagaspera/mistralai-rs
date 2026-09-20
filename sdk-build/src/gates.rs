use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub(crate) fn fail<T>(message: impl Into<String>) -> Result<T> {
    Err(std::io::Error::other(message.into()).into())
}

pub(crate) fn read_json(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

pub(crate) fn write_json(path: &Path, value: &Value) -> Result<()> {
    fs::write(path, format!("{}\n", serde_json::to_string_pretty(value)?))?;
    Ok(())
}

pub(crate) fn field<'a>(value: &'a Value, name: &str) -> Result<&'a Value> {
    value
        .get(name)
        .ok_or_else(|| std::io::Error::other(format!("missing field {name}")).into())
}

pub(crate) fn string<'a>(value: &'a Value, name: &str) -> Result<&'a str> {
    field(value, name)?
        .as_str()
        .ok_or_else(|| std::io::Error::other(format!("{name} is not a string")).into())
}

fn source_operations(spec: &Value) -> Result<BTreeMap<String, Value>> {
    let mut source = BTreeMap::new();
    let paths = field(spec, "paths")?
        .as_object()
        .ok_or("OpenAPI paths must be an object")?;
    for (path, item) in paths {
        let Some(methods) = item.as_object() else {
            continue;
        };
        for (method, operation) in methods {
            if !matches!(
                method.to_ascii_lowercase().as_str(),
                "get"
                    | "put"
                    | "post"
                    | "delete"
                    | "patch"
                    | "head"
                    | "options"
                    | "trace"
                    | "query"
            ) {
                continue;
            }
            let Some(op) = operation.as_object() else {
                continue;
            };
            let Some(id) = op
                .get("operationId")
                .and_then(Value::as_str)
                .filter(|id| !id.is_empty())
            else {
                continue;
            };
            let mut media = BTreeSet::new();
            if let Some(responses) = op.get("responses").and_then(Value::as_object) {
                for (status, response) in responses {
                    if !status.starts_with('2') {
                        continue;
                    }
                    if let Some(content) = response.get("content").and_then(Value::as_object) {
                        media.extend(content.keys().cloned());
                    }
                }
            }
            if source
                .insert(
                    id.into(),
                    json!({
                        "method": method.to_ascii_uppercase(),
                        "operation_id": id,
                        "path": path.split('#').next().unwrap_or(path),
                        "success_media": media,
                        "tags": op.get("tags").cloned().unwrap_or_else(|| json!([])),
                        "upstream": true
                    }),
                )
                .is_some()
            {
                return fail(format!("duplicate OpenAPI operation ID: {id}"));
            }
        }
    }
    Ok(source)
}

pub(crate) fn source_paths(spec: &Value) -> Result<BTreeMap<String, String>> {
    let paths = field(spec, "paths")?
        .as_object()
        .ok_or("OpenAPI paths must be an object")?;
    let mut result = BTreeMap::new();
    for (path, methods) in paths {
        let Some(methods) = methods.as_object() else {
            continue;
        };
        for (method, operation) in methods {
            if !matches!(
                method.to_ascii_lowercase().as_str(),
                "get"
                    | "put"
                    | "post"
                    | "delete"
                    | "patch"
                    | "head"
                    | "options"
                    | "trace"
                    | "query"
            ) {
                continue;
            }
            let Some(id) = operation
                .get("operationId")
                .and_then(Value::as_str)
                .filter(|id| !id.is_empty())
            else {
                continue;
            };
            if let Some(previous) = result.insert(id.into(), path.clone()) {
                if previous != *path {
                    return fail(format!(
                        "OpenAPI operationId {id} appears at both {previous} and {path}"
                    ));
                }
            }
        }
    }
    Ok(result)
}

pub(crate) fn raw_coverage(generated: &Path, spec: &Value) -> Result<Value> {
    let manifest = read_json(&generated.join("binding-manifest.json"))?;
    let source = source_operations(spec)?;
    let operations = field(&manifest, "operations")?
        .as_array()
        .ok_or("binding operations must be an array")?;
    let mut emitted = BTreeSet::new();
    for binding in operations {
        if binding.get("kind").and_then(Value::as_str) != Some("call_shape") {
            continue;
        }
        let origin = field(binding, "source_operation")?;
        let id = string(origin, "operation_id")?;
        let method = string(origin, "method")?;
        let path = string(origin, "path")?.split('#').next().unwrap_or("");
        let Some(expected) = source.get(id) else {
            return fail(format!("raw binding operation missing from OpenAPI: {id}"));
        };
        if expected["method"] != method || expected["path"] != path {
            return fail(format!(
                "raw binding operation disagrees with OpenAPI: {id}"
            ));
        }
        emitted.insert(id.to_owned());
    }
    let expected: BTreeSet<_> = source.keys().cloned().collect();
    if emitted != expected {
        return fail(format!(
            "raw binding coverage disagrees with OpenAPI: missing={:?}; extra={:?}",
            expected.difference(&emitted).collect::<Vec<_>>(),
            emitted.difference(&expected).collect::<Vec<_>>()
        ));
    }
    Ok(json!({
        "generated_methods": operations.len(),
        "operations": source.into_values().collect::<Vec<_>>(),
        "upstream_operations": emitted.len()
    }))
}

fn identities(coverage: &Value) -> Result<BTreeMap<String, (String, String)>> {
    let mut result = BTreeMap::new();
    let operations = field(coverage, "operations")?
        .as_array()
        .ok_or("raw coverage operations must be an array")?;
    for operation in operations {
        let id = string(operation, "operation_id")?;
        if result
            .insert(
                id.into(),
                (
                    string(operation, "method")?.into(),
                    string(operation, "path")?.into(),
                ),
            )
            .is_some()
        {
            return fail(format!("duplicate raw coverage operation: {id}"));
        }
    }
    Ok(result)
}

pub(crate) fn verify_raw_coverage(committed: &Value, regenerated: &Value) -> Result<()> {
    if identities(committed)? != identities(regenerated)? {
        return fail("committed raw coverage operation identities differ from generated bindings");
    }
    Ok(())
}

pub(crate) fn snapshot(dir: &Path) -> Result<BTreeMap<String, Vec<u8>>> {
    fn visit(root: &Path, dir: &Path, files: &mut BTreeMap<String, Vec<u8>>) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                visit(root, &path, files)?;
            } else if path.is_file() {
                let relative = path
                    .strip_prefix(root)?
                    .to_string_lossy()
                    .replace('\\', "/");
                files.insert(relative, fs::read(path)?);
            }
        }
        Ok(())
    }
    let mut files = BTreeMap::new();
    visit(dir, dir, &mut files)?;
    Ok(files)
}

fn normalized_manifest(data: &[u8], operation_paths: &BTreeMap<String, String>) -> Result<Value> {
    let mut value: Value = serde_json::from_slice(data)?;
    if let Some(operations) = value.get_mut("operations").and_then(Value::as_array_mut) {
        for operation in operations {
            let Some(origin) = operation
                .get_mut("source_operation")
                .and_then(Value::as_object_mut)
            else {
                continue;
            };
            let id = origin
                .get("operation_id")
                .and_then(Value::as_str)
                .ok_or("manifest operation missing operation_id")?;
            let Some(path) = operation_paths.get(id) else {
                return fail(format!("manifest operation {id} missing from OpenAPI"));
            };
            origin.insert("path".into(), json!(path));
        }
    }
    Ok(value)
}

pub(crate) fn verify_raw_baseline(generated: &Path, committed: &Path, spec: &Value) -> Result<()> {
    let mut actual = snapshot(generated)?;
    let mut expected = snapshot(committed)?;
    actual.remove("coverage.json");
    expected.remove("coverage.json");
    if actual.keys().collect::<Vec<_>>() != expected.keys().collect::<Vec<_>>() {
        return fail(format!(
            "raw file inventory drifted: expected={:?}; actual={:?}",
            expected.keys().collect::<Vec<_>>(),
            actual.keys().collect::<Vec<_>>()
        ));
    }
    let paths = source_paths(spec)?;
    for (name, after) in actual {
        let before = &expected[&name];
        if name == "binding-manifest.json" {
            if normalized_manifest(before, &paths)? != normalized_manifest(&after, &paths)? {
                return fail("binding manifest drifted beyond exact source-operation path repair");
            }
        } else {
            let normalized = String::from_utf8_lossy(before)
                .replace("../sources/openapi/openapi.yaml", "openapi/published.yaml");
            if normalized.as_bytes() != after {
                return fail(format!(
                    "raw output drifted beyond source provenance marker: {name}"
                ));
            }
        }
    }
    Ok(())
}

pub(crate) fn verify_overlaid(spec: &Value) -> Result<()> {
    let schemas = &spec["components"]["schemas"];
    let required = |value: &Value, needle: &str| {
        value
            .get("required")
            .and_then(Value::as_array)
            .is_some_and(|items| items.iter().any(|item| item.as_str() == Some(needle)))
    };
    if required(&schemas["ChatCompletionResponse"]["allOf"][1], "data")
        || required(&schemas["SharingDelete"], "level")
        || required(&schemas["WorkflowListResponse"], "beta.workflows")
        || !required(&schemas["WorkflowListResponse"], "workflows")
    {
        return fail("Mistral OpenAPI overlay assertions failed");
    }
    let paths = field(spec, "paths")?
        .as_object()
        .ok_or("OpenAPI paths must be an object")?;
    let actual: BTreeSet<_> = paths
        .keys()
        .filter(|p| p.contains("#stream") || p.contains("#wav"))
        .map(String::as_str)
        .collect();
    let expected: BTreeSet<_> = [
        "/v1/conversations#stream",
        "/v1/conversations/{conversation_id}#stream",
        "/v1/conversations/{conversation_id}/restart#stream",
        "/v1/audio/transcriptions#stream",
    ]
    .into_iter()
    .collect();
    if actual != expected {
        return fail(format!(
            "overlay changed published representation-specific path inventory: {actual:?}"
        ));
    }
    Ok(())
}

pub(crate) fn validate_coverage(
    report: &Value,
    baseline: &Value,
    spec: &Value,
) -> Result<BTreeMap<String, usize>> {
    if baseline["schema_version"] != 1 {
        return fail("unsupported SDK coverage baseline schema");
    }
    let sources: BTreeSet<_> = source_paths(spec)?.into_keys().collect();
    let operations = field(report, "operations")?
        .as_object()
        .ok_or("SDK derivation report missing operations")?;
    let actual: BTreeSet<_> = operations.keys().cloned().collect();
    if sources.len() != baseline["total_operations"].as_u64().unwrap_or(0) as usize
        || sources != actual
    {
        return fail(format!(
            "SDK operation inventory drift: source={}, report={}, expected={}; missing={:?}; extra={:?}",
            sources.len(),
            actual.len(),
            baseline["total_operations"],
            sources.difference(&actual).collect::<Vec<_>>(),
            actual.difference(&sources).collect::<Vec<_>>()
        ));
    }
    let list = |name: &str| -> Result<Vec<String>> {
        Ok(field(baseline, name)?
            .as_array()
            .ok_or("coverage baseline list must be an array")?
            .iter()
            .map(|v| {
                v.as_str()
                    .map(str::to_owned)
                    .ok_or("coverage identity must be a string")
            })
            .collect::<std::result::Result<Vec<_>, _>>()?)
    };
    let rejected = list("previously_rejected_operations")?;
    let overridden = list("approved_overridden_operations")?;
    let allowed_rejected: BTreeSet<_> = rejected.iter().cloned().collect();
    let allowed_overridden: BTreeSet<_> = overridden.iter().cloned().collect();
    if rejected.len() != allowed_rejected.len()
        || overridden.len() != allowed_overridden.len()
        || !allowed_rejected.is_disjoint(&allowed_overridden)
        || !allowed_rejected
            .union(&allowed_overridden)
            .all(|id| sources.contains(id))
    {
        return fail("SDK coverage baseline has invalid or unknown operation identities");
    }
    let mut counts = BTreeMap::new();
    let mut new_rejections = Vec::new();
    let mut new_overrides = Vec::new();
    for (id, item) in operations {
        let status = string(item, "status")?;
        if !matches!(status, "derived" | "overridden" | "rejected") {
            return fail(format!("unknown SDK derivation status: {status}"));
        }
        *counts.entry(status.to_owned()).or_insert(0) += 1;
        if status == "rejected" && !allowed_rejected.contains(id) {
            new_rejections.push(id);
        }
        if status == "overridden" && !allowed_overridden.contains(id) {
            new_overrides.push(id);
        }
    }
    if !new_rejections.is_empty() || !new_overrides.is_empty() {
        return fail(format!(
            "SDK coverage regression: new_rejections={new_rejections:?}; unreviewed_overrides={new_overrides:?}"
        ));
    }
    Ok(counts)
}

pub(crate) fn require_publish_parity(
    counts: &BTreeMap<String, usize>,
    delta: &[String],
) -> Result<()> {
    if counts.get("rejected").copied().unwrap_or(0) > 0 {
        return fail(format!(
            "cannot publish SDK: {} operations remain rejected",
            counts["rejected"]
        ));
    }
    if !delta.is_empty() {
        return fail(format!(
            "cannot publish SDK: generated SDK facade differs from committed public baseline in {} files: {}",
            delta.len(),
            delta.join(", ")
        ));
    }
    Ok(())
}

pub(crate) fn facade_delta(
    actual: &BTreeMap<String, Vec<u8>>,
    committed: &BTreeMap<String, Vec<u8>>,
) -> Vec<String> {
    actual
        .keys()
        .chain(committed.keys())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .filter(|name| actual.get(*name) != committed.get(*name))
        .cloned()
        .collect()
}

pub(crate) fn committed_facade(path: &Path) -> Result<BTreeMap<String, Vec<u8>>> {
    Ok(snapshot(path)?
        .into_iter()
        .filter(|(name, _)| name.ends_with(".rs") && name != "error.rs")
        .collect())
}

pub(crate) fn copy_dir(source: &Path, target: &Path) -> Result<()> {
    fs::create_dir_all(target)?;
    for entry in fs::read_dir(source)? {
        let path: PathBuf = entry?.path();
        let dest = target.join(path.file_name().ok_or("source file without a name")?);
        if path.is_dir() {
            copy_dir(&path, &dest)?;
        } else if path.is_file() {
            fs::copy(path, dest)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_missing_operation_and_unreviewed_override() {
        let spec =
            json!({"paths":{"/x":{"get":{"operationId":"a"}}, "/y":{"post":{"operationId":"b"}}}});
        let baseline = json!({"schema_version":1,"total_operations":2,"previously_rejected_operations":[],"approved_overridden_operations":[]});
        let report = json!({"operations":{"a":{"status":"derived"}}});
        assert!(validate_coverage(&report, &baseline, &spec).is_err());
        let report = json!({"operations":{"a":{"status":"derived"},"b":{"status":"overridden"}}});
        assert!(validate_coverage(&report, &baseline, &spec).is_err());
    }
    #[test]
    fn exact_facade_parity_is_independent_of_coverage() {
        let counts = BTreeMap::from([("derived".into(), 173)]);
        assert!(require_publish_parity(&counts, &[]).is_ok());
        assert!(require_publish_parity(&counts, &["client.rs".into()]).is_err());
        assert!(require_publish_parity(&BTreeMap::from([("rejected".into(), 1)]), &[]).is_err());
    }
    #[test]
    fn coverage_gates_reject_unknown_status_rejection_and_duplicate_review() {
        let spec = json!({"paths":{"/a":{"get":{"operationId":"a"}},
                                     "/b":{"post":{"operationId":"b"}}}});
        let baseline = json!({"schema_version":1,"total_operations":2,
            "previously_rejected_operations":[],"approved_overridden_operations":[]});
        let mut report = json!({"operations":{"a":{"status":"derived"},
                                                  "b":{"status":"derived"}}});
        assert_eq!(validate_coverage(&report, &baseline, &spec).unwrap()["derived"], 2);
        report["operations"]["b"]["status"] = json!("skipped");
        assert!(validate_coverage(&report, &baseline, &spec).is_err());
        report["operations"]["b"]["status"] = json!("rejected");
        assert!(validate_coverage(&report, &baseline, &spec).is_err());
        let invalid = json!({"schema_version":1,"total_operations":2,
            "previously_rejected_operations":["a","a"],"approved_overridden_operations":[]});
        assert!(validate_coverage(&report, &invalid, &spec).is_err());
    }

    #[test]
    fn source_identities_and_raw_inventory_fail_closed() {
        let duplicate = json!({"paths":{
            "/a":{"get":{"operationId":"same"}},
            "/b":{"post":{"operationId":"same"}}
        }});
        assert!(source_paths(&duplicate).is_err());
        assert!(source_operations(&duplicate).is_err());
        let actual = json!({"operations":[{"operation_id":"one","method":"POST","path":"/a"}]});
        let drifted = json!({"operations":[{"operation_id":"one","method":"GET","path":"/a"}]});
        verify_raw_coverage(&actual, &actual).unwrap();
        assert!(verify_raw_coverage(&actual, &drifted).is_err());
        let duplicate_inventory = json!({"operations":[
            {"operation_id":"one","method":"POST","path":"/a"},
            {"operation_id":"one","method":"POST","path":"/a"}
        ]});
        assert!(verify_raw_coverage(&duplicate_inventory, &actual).is_err());
    }

    #[test]
    fn facade_gate_compares_entire_inventory_and_exact_bytes() {
        let expected = BTreeMap::from([
            ("client.rs".into(), b"original".to_vec()),
            ("models.rs".into(), b"model".to_vec()),
        ]);
        assert!(facade_delta(&expected, &expected).is_empty());
        let mut actual = expected.clone();
        actual.insert("client.rs".into(), b"modified".to_vec());
        actual.remove("models.rs");
        actual.insert("extra.rs".into(), Vec::new());
        assert_eq!(
            facade_delta(&actual, &expected),
            vec!["client.rs", "extra.rs", "models.rs"]
        );
    }

    #[test]
    fn raw_coverage_rejects_missing_binding_or_incorrect_path() {
        let temp = std::env::temp_dir().join(format!(
            "mistralai-sdk-build-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&temp).unwrap();
        let spec = json!({"paths":{"/v1/echo#stream":{"post":{
            "operationId":"echo_stream","tags":["test"],
            "responses":{"200":{"content":{"text/event-stream":{}}}}
        }}}});
        let mut manifest = json!({"operations":[]});
        write_json(&temp.join("binding-manifest.json"), &manifest).unwrap();
        assert!(raw_coverage(&temp, &spec).is_err());
        manifest["operations"] = json!([{
            "kind":"call_shape",
            "source_operation":{"operation_id":"echo_stream",
                "method":"POST","path":"/v1/wrong#stream"}
        }]);
        write_json(&temp.join("binding-manifest.json"), &manifest).unwrap();
        assert!(raw_coverage(&temp, &spec).is_err());
        manifest["operations"][0]["source_operation"]["path"] = json!("/v1/echo#stream");
        write_json(&temp.join("binding-manifest.json"), &manifest).unwrap();
        let coverage = raw_coverage(&temp, &spec).unwrap();
        assert_eq!(coverage["upstream_operations"], 1);
        assert_eq!(coverage["operations"][0]["path"], "/v1/echo");
        fs::remove_dir_all(temp).unwrap();
    }

    #[test]
    fn raw_coverage_tracks_source_operations_not_variants() {
        let spec = json!({"paths":{"/x#stream":{"post":{"operationId":"a","responses":{"200":{"content":{"text/event-stream":{}}}}}}}});
        let source = source_operations(&spec).unwrap();
        assert_eq!(source.len(), 1);
        assert_eq!(source["a"]["path"], "/x");
    }
}
