mod gates;
mod sources;

use gates::{
    Result, committed_facade, copy_dir, facade_delta, fail, field, raw_coverage, read_json,
    require_publish_parity, snapshot, string, validate_coverage, verify_bindings_coverage,
    verify_candidate_overlaid, verify_overlaid, verify_raw_baseline, verify_raw_coverage,
    write_json,
};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn run(program: impl AsRef<OsStr>, args: Vec<String>, cwd: &Path) -> Result<()> {
    let status = Command::new(program)
        .args(&args)
        .current_dir(cwd)
        .status()?;
    if !status.success() {
        return fail(format!("command failed ({status}): {args:?}"));
    }
    Ok(())
}

fn output(program: impl AsRef<OsStr>, args: Vec<String>, cwd: &Path) -> Result<String> {
    let output = Command::new(program)
        .args(&args)
        .current_dir(cwd)
        .output()?;
    if !output.status.success() {
        return fail(format!(
            "command failed ({}): {args:?}\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

fn root() -> Result<PathBuf> {
    // CARGO_MANIFEST_DIR is absolute during cargo builds; no dependence on cwd.
    Ok(Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("sdk-build has no repository parent")?
        .to_path_buf())
}

fn str_arg(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn check_toolchain(root: &Path, lock: &Value) -> Result<String> {
    let version = string(lock, "rust_toolchain")?.to_owned();
    let toolchain = fs::read_to_string(root.join("rust-toolchain.toml"))?;
    let configured = toolchain
        .lines()
        .find_map(|line| line.trim().strip_prefix("channel = "))
        .and_then(|line| line.trim().strip_prefix('"')?.split('"').next())
        .ok_or("rust-toolchain.toml missing channel")?;
    if configured != version {
        return fail("rust-toolchain.toml and provenance lock disagree");
    }
    let installed = output("rustup", vec!["toolchain".into(), "list".into()], root)?;
    if !installed.lines().any(|line| {
        line.split_whitespace()
            .next()
            .is_some_and(|name| name == version || name.starts_with(&format!("{version}-")))
    }) {
        run(
            "rustup",
            vec![
                "toolchain".into(),
                "install".into(),
                version.clone(),
                "--profile".into(),
                "minimal".into(),
                "--component".into(),
                "rustfmt".into(),
                "--component".into(),
                "clippy".into(),
            ],
            root,
        )?;
    }
    Ok(version)
}

fn checkout_tool(root: &Path, tool: &Value) -> Result<PathBuf> {
    let name = string(tool, "name")?;
    let revision = string(tool, "commit")?;
    let repository = string(tool, "repository")?;
    let source = root.join(".tools").join(format!("{name}-{revision}"));
    if !source.join(".git").exists() {
        if source.exists() {
            fs::remove_dir_all(&source)?;
        }
        fs::create_dir_all(&source)?;
        run("git", vec!["init".into(), str_arg(&source)], root)?;
        run(
            "git",
            vec![
                "fetch".into(),
                "--depth=1".into(),
                format!("https://github.com/{repository}.git"),
                revision.into(),
            ],
            &source,
        )?;
        run(
            "git",
            vec!["checkout".into(), "--detach".into(), "FETCH_HEAD".into()],
            &source,
        )?;
    }
    if output("git", vec!["rev-parse".into(), "HEAD".into()], &source)? != revision {
        return fail(format!("{name} commit mismatch"));
    }
    if let Some(tree) = tool.get("tree_sha").and_then(Value::as_str) {
        let actual_tree = output(
            "git",
            vec!["rev-parse".into(), "HEAD^{tree}".into()],
            &source,
        )?;
        if actual_tree != tree {
            return fail(format!("{name} tree mismatch"));
        }
    }
    if !output("git", vec!["status".into(), "--porcelain".into()], &source)?.is_empty() {
        return fail(format!("{name} pinned checkout has local modifications"));
    }
    if let Some(subdir) = tool.get("subdirectory").and_then(Value::as_str) {
        let subdirectory = source.join(subdir);
        if !subdirectory.is_dir() {
            return fail(format!("{name} pinned subdirectory missing"));
        }
        Ok(subdirectory)
    } else {
        Ok(source)
    }
}

fn install_tool(root: &Path, lock: &Value, key: &str, binary: &str) -> Result<PathBuf> {
    let tool = field(field(lock, "tools")?, key)?;
    let source = checkout_tool(root, tool)?;
    let install = root.join(".tools").join(format!(
        "{}-{}-install",
        string(tool, "name")?,
        string(tool, "commit")?
    ));
    let executable = install.join("bin").join(binary);
    if !executable.is_file() {
        run(
            "cargo",
            vec![
                format!("+{}", string(lock, "rust_toolchain")?),
                "install".into(),
                "--locked".into(),
                "--path".into(),
                str_arg(&source),
                "--root".into(),
                str_arg(&install),
            ],
            root,
        )?;
    }
    if binary == "rust-sdk-generator" {
        // This pinned CLI exposes --help but no --version. Its immutable commit
        // and tree are checked above; do not invent an unsupported CLI contract.
        let help = output(&executable, vec!["--help".into()], root)?;
        if !help.contains("rust-sdk-generator derive") {
            return fail("unexpected rust-sdk-generator CLI");
        }
    } else {
        let expected = format!("{} {}", binary, string(tool, "version")?);
        let actual = output(&executable, vec!["--version".into()], root)?;
        if actual != expected {
            return fail(format!(
                "unexpected {binary}: expected {expected}, got {actual}"
            ));
        }
    }
    Ok(executable)
}

struct Workspace(PathBuf);
impl Workspace {
    fn new(root: &Path) -> Result<Self> {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let path = root.join(format!(".sdk-build-probe-{}-{nanos}", std::process::id()));
        fs::create_dir(&path)?;
        Ok(Self(path))
    }
}
impl Drop for Workspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn copy_inputs(root: &Path, work: &Path) -> Result<()> {
    let from = root.join("sdk-build");
    let to = work.join("sdk-build");
    fs::create_dir_all(&to)?;
    copy_dir(&from.join("openapi"), &to.join("openapi"))?;
    copy_dir(&from.join("official-sdks"), &to.join("official-sdks"))?;
    for file in [
        "openapi-to-rust.toml",
        "sdk-overrides.json",
        "sdk-overrides-candidate.json",
        "coverage-baseline.json",
        "candidate-derivation-baseline.json",
    ] {
        fs::copy(from.join(file), to.join(file))?;
    }
    Ok(())
}

fn rust_files(dir: &Path) -> Result<Vec<PathBuf>> {
    fn visit(dir: &Path, paths: &mut Vec<PathBuf>) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                visit(&path, paths)?;
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                paths.push(path);
            }
        }
        Ok(())
    }
    let mut paths = Vec::new();
    visit(dir, &mut paths)?;
    paths.sort();
    Ok(paths)
}

fn format_rust(root: &Path, version: &str, dir: &Path) -> Result<()> {
    for file in rust_files(dir)? {
        run(
            "rustup",
            vec![
                "run".into(),
                version.into(),
                "rustfmt".into(),
                "--edition".into(),
                "2024".into(),
                "--config".into(),
                "skip_children=true".into(),
                str_arg(&file),
            ],
            root,
        )?;
    }
    Ok(())
}

fn normalized_coverage(root: &Path, generated: &Path, spec: &Value) -> Result<()> {
    let coverage = raw_coverage(generated, spec)?;
    write_json(&generated.join("coverage.json"), &coverage)?;
    verify_raw_baseline(generated, &root.join("src/generated"), spec)?;
    verify_raw_coverage(
        &read_json(&root.join("src/generated/coverage.json"))?,
        &coverage,
    )
}

fn compatible_snapshot(
    root: &Path,
    generated: &Path,
    committed: &BTreeMap<String, Vec<u8>>,
) -> Result<Vec<String>> {
    let _ = root;
    Ok(facade_delta(&snapshot(generated)?, committed))
}

fn stage_directory(source: &Path, target: &Path, backup: &Path) -> Result<()> {
    if backup.exists() {
        return fail(format!("stale publication backup: {}", backup.display()));
    }
    if !target.is_dir() {
        return fail(format!("publication target missing: {}", target.display()));
    }
    fs::rename(target, backup)?;
    if let Err(error) = fs::rename(source, target) {
        let _ = fs::rename(backup, target);
        return Err(error.into());
    }
    Ok(())
}

fn rollback_directory(target: &Path, backup: &Path) -> Result<()> {
    fs::remove_dir_all(target)?;
    fs::rename(backup, target)?;
    Ok(())
}

fn publish(root: &Path, work: &Path, generated: &Path, compatible: &Path) -> Result<()> {
    // Both outputs are fully prepared and validated before touching the checkout.
    let raw_target = root.join("src/generated");
    let sdk_target = root.join("src/sdk");
    let raw_stage = work.join("raw-publication");
    let sdk_stage = work.join("sdk-publication");
    copy_dir(generated, &raw_stage)?;
    copy_dir(&sdk_target, &sdk_stage)?;
    let old_generated: BTreeSet<_> = committed_facade(&sdk_stage)?.into_keys().collect();
    let replacement = snapshot(compatible)?;
    for name in old_generated {
        if !replacement.contains_key(&name) {
            fs::remove_file(sdk_stage.join(name))?;
        }
    }
    for (name, bytes) in replacement {
        fs::write(sdk_stage.join(name), bytes)?;
    }
    let raw_backup = work.join("raw-backup");
    let sdk_backup = work.join("sdk-backup");
    stage_directory(&raw_stage, &raw_target, &raw_backup)?;
    if let Err(error) = stage_directory(&sdk_stage, &sdk_target, &sdk_backup) {
        rollback_directory(&raw_target, &raw_backup)?;
        return Err(error);
    }
    // Both outputs are committed; cleanup must not report failure after publication.
    let _ = fs::remove_dir_all(&raw_backup);
    let _ = fs::remove_dir_all(&sdk_backup);
    println!("Published verified raw bindings and exactly compatible SDK facade.");
    Ok(())
}

fn candidate_config(build: &Path) -> Result<PathBuf> {
    let source = build.join("openapi-to-rust.toml");
    let mut config = fs::read_to_string(&source)?;
    for (before, after) in [
        (
            "spec_path = \"openapi/published.yaml\"",
            "spec_path = \"openapi/public-288.yaml\"",
        ),
        (
            "overlays = [\"openapi/overlays/rust-sdk.overlay.yaml\"]",
            "overlays = [\"openapi/overlays/public-288.overlay.yaml\"]",
        ),
        (
            "overlay_output = \"openapi/overlaid.json\"",
            "overlay_output = \"openapi/overlaid-candidate.json\"",
        ),
        (
            "output_dir = \"../src/generated\"",
            "output_dir = \"../candidate-generated\"",
        ),
    ] {
        if !config.contains(before) {
            return fail(format!("candidate raw config anchor missing: {before}"));
        }
        config = config.replacen(before, after, 1);
    }
    let path = build.join("openapi-to-rust-candidate.toml");
    fs::write(&path, config)?;
    Ok(path)
}

fn compile_standalone_raw(root: &Path, version: &str, generated: &Path, work: &Path) -> Result<()> {
    let crate_dir = work.join("candidate-raw-compile");
    let crate_src = crate_dir.join("src");
    fs::create_dir_all(&crate_src)?;
    copy_dir(generated, &crate_src.join("generated"))?;
    fs::write(
        crate_src.join("lib.rs"),
        "pub mod generated;\npub use generated::*;\n",
    )?;
    let dependencies = fs::read_to_string(generated.join("REQUIRED_DEPS.toml"))?;
    let mut manifest = format!(
        "[package]\nname = \"mistralai-candidate-raw-check\"\nversion = \"0.0.0\"\nedition = \"2024\"\nrust-version = \"{version}\"\n\n[workspace]\n\n{dependencies}"
    );
    // Test-only runtime; the raw generator's REQUIRED_DEPS.toml stays verbatim.
    manifest.push_str(
        "\n[dev-dependencies]\ntokio = { version = \"1\", features = [\"macros\", \"rt-multi-thread\"] }\n",
    );
    fs::write(crate_dir.join("Cargo.toml"), manifest)?;
    run(
        "cargo",
        vec![
            format!("+{version}"),
            "check".into(),
            "--manifest-path".into(),
            str_arg(&crate_dir.join("Cargo.toml")),
        ],
        root,
    )?;
    let tests = crate_dir.join("tests");
    fs::create_dir_all(&tests)?;
    fs::copy(
        root.join("sdk-build/fixtures/candidate-raw-nullable.rs"),
        tests.join("request_json.rs"),
    )?;
    fs::copy(
        root.join("sdk-build/fixtures/candidate-raw-parameter-wire.rs"),
        tests.join("parameter_wire.rs"),
    )?;
    fs::copy(
        root.join("sdk-build/fixtures/candidate-raw-optional-nullable-body.rs"),
        tests.join("optional_nullable_body.rs"),
    )?;
    fs::copy(
        root.join("sdk-build/fixtures/candidate-raw-required-json-root.rs"),
        tests.join("required_json_root.rs"),
    )?;
    fs::copy(
        root.join("sdk-build/fixtures/candidate-raw-typed-sse.rs"),
        tests.join("typed_sse.rs"),
    )?;
    run(
        "cargo",
        vec![
            format!("+{version}"),
            "test".into(),
            "--manifest-path".into(),
            str_arg(&crate_dir.join("Cargo.toml")),
            "--test".into(),
            "request_json".into(),
            "--test".into(),
            "parameter_wire".into(),
            "--test".into(),
            "optional_nullable_body".into(),
            "--test".into(),
            "required_json_root".into(),
            "--test".into(),
            "typed_sse".into(),
        ],
        root,
    )
}

fn verify_candidate_raw(
    root: &Path,
    lock: &Value,
    version: &str,
    raw: &Path,
    bindings: &Path,
) -> Result<()> {
    let temp = Workspace::new(root)?;
    let work = &temp.0;
    copy_inputs(root, work)?;
    let build = work.join("sdk-build");
    let config = candidate_config(&build)?;
    let published = build.join("openapi/public-288.yaml");
    let overlaid = build.join("openapi/overlaid-candidate.json");
    let generated = work.join("candidate-generated");
    let raw_args = vec!["generate".into(), "--config".into(), str_arg(&config)];

    run(raw, raw_args.clone(), root)?;
    let first = fs::read(&overlaid)?;
    let spec: Value = serde_json::from_slice(&first)?;
    verify_candidate_overlaid(&spec)?;
    run(raw, raw_args.clone(), root)?;
    if fs::read(&overlaid)? != first {
        return fail("candidate overlaid OpenAPI is not byte-for-byte deterministic");
    }
    let mut check = raw_args.clone();
    check.push("--check".into());
    run(raw, check, root)?;
    let mut dry_run = raw_args;
    dry_run.push("--dry-run".into());
    run(raw, dry_run, root)?;
    if fs::read(&published)? != fs::read(root.join("sdk-build/openapi/public-288.yaml"))? {
        return fail("candidate raw generation modified the staged OpenAPI");
    }

    format_rust(root, version, &generated)?;
    let coverage = raw_coverage(&generated, &spec)?;
    let candidate = field(lock, "openapi_candidate")?;
    let expected_operations = candidate["operations"]
        .as_u64()
        .ok_or("candidate expected operation count is missing")?;
    let expected_methods = candidate["generated_methods"]
        .as_u64()
        .ok_or("candidate expected generated-method count is missing")?;
    if coverage["upstream_operations"].as_u64() != Some(expected_operations)
        || coverage["generated_methods"].as_u64() != Some(expected_methods)
    {
        return fail(format!(
            "candidate raw inventory drift: source={}, methods={}, expected={expected_operations}/{expected_methods}",
            coverage["upstream_operations"], coverage["generated_methods"]
        ));
    }
    write_json(&generated.join("coverage.json"), &coverage)?;
    compile_standalone_raw(root, version, &generated, work)?;

    let bindings_value: Value = serde_json::from_str(&output(
        bindings,
        vec![str_arg(&generated), str_arg(&overlaid)],
        root,
    )?)?;
    let binding_report = verify_bindings_coverage(&bindings_value, &spec)?;
    if binding_report["binding_operations"].as_u64() != Some(expected_methods) {
        return fail(format!(
            "candidate normalized Bindings method inventory drift: {} != {expected_methods}",
            binding_report["binding_operations"]
        ));
    }
    if binding_report["representation_path_pairs"].as_u64() != Some(4) {
        return fail(format!(
            "candidate representation-specific path pair inventory drift: {} != 4",
            binding_report["representation_path_pairs"]
        ));
    }

    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "candidate_raw": {
                "coverage": coverage,
                "bindings": binding_report,
                "standalone_compile": "ok"
            }
        }))?
    );
    Ok(())
}

fn verify_candidate_derivation(
    root: &Path,
    lock: &Value,
    version: &str,
    raw: &Path,
    bindings: &Path,
    compiler: &Path,
) -> Result<()> {
    let temp = Workspace::new(root)?;
    let work = &temp.0;
    copy_inputs(root, work)?;
    let build = work.join("sdk-build");
    let config = candidate_config(&build)?;
    let overlaid = build.join("openapi/overlaid-candidate.json");
    let generated = work.join("candidate-generated");
    run(
        raw,
        vec!["generate".into(), "--config".into(), str_arg(&config)],
        root,
    )?;
    let spec: Value = serde_json::from_slice(&fs::read(&overlaid)?)?;
    verify_candidate_overlaid(&spec)?;
    format_rust(root, version, &generated)?;

    let coverage = raw_coverage(&generated, &spec)?;
    let candidate = field(lock, "openapi_candidate")?;
    let expected_operations = candidate["operations"]
        .as_u64()
        .ok_or("candidate expected operation count is missing")?;
    let expected_methods = candidate["generated_methods"]
        .as_u64()
        .ok_or("candidate expected generated-method count is missing")?;
    if coverage["upstream_operations"].as_u64() != Some(expected_operations)
        || coverage["generated_methods"].as_u64() != Some(expected_methods)
    {
        return fail("candidate derivation probe raw inventory drifted");
    }

    let bindings_value: Value = serde_json::from_str(&output(
        bindings,
        vec![str_arg(&generated), str_arg(&overlaid)],
        root,
    )?)?;
    verify_bindings_coverage(&bindings_value, &spec)?;
    let bindings_path = work.join("bindings.json");
    write_json(&bindings_path, &bindings_value)?;

    let derivation: Value = serde_json::from_str(&output(
        compiler,
        vec![
            "derive".into(),
            "--openapi".into(),
            str_arg(&overlaid),
            "--bindings".into(),
            str_arg(&bindings_path),
            "--surface".into(),
            str_arg(&build.join("official-sdks/surface.json")),
            "--overrides".into(),
            str_arg(&build.join("sdk-overrides-candidate.json")),
        ],
        root,
    )?)?;
    let report = field(&derivation, "report")?;
    let operations = field(report, "operations")?
        .as_object()
        .ok_or("candidate derivation report operations must be an object")?;
    if operations.len() as u64 != expected_operations {
        return fail(format!(
            "candidate derivation report inventory drift: {} != {expected_operations}",
            operations.len()
        ));
    }

    let mut statuses = BTreeMap::<String, usize>::new();
    let mut rejections = BTreeMap::<String, Vec<String>>::new();
    let mut override_targets = BTreeMap::<String, Value>::new();
    let configured_overrides = read_json(&build.join("sdk-overrides-candidate.json"))?;
    let configured = field(&configured_overrides, "operations")?
        .as_object()
        .ok_or("candidate overrides operations must be an object")?;

    for (operation_id, outcome) in operations {
        let status = string(outcome, "status")?;
        *statuses.entry(status.to_owned()).or_default() += 1;
        let reason = field(outcome, "reason")?;
        let reason_code = string(reason, "code")?;
        if status == "rejected" {
            rejections
                .entry(reason_code.to_owned())
                .or_default()
                .push(operation_id.clone());
        }
        if configured.contains_key(operation_id) {
            override_targets.insert(
                operation_id.clone(),
                json!({
                    "status": status,
                    "reason": reason
                }),
            );
        }
    }

    let target = root.join("sdk-build/target");
    fs::create_dir_all(&target)?;
    write_json(&target.join("candidate-bindings.json"), &bindings_value)?;
    fs::copy(&overlaid, target.join("candidate-overlaid.json"))?;
    write_json(&target.join("candidate-derivation-report.json"), report)?;
    write_json(
        &target.join("candidate-sdk-definition.json"),
        field(&derivation, "definition")?,
    )?;
    let observed = json!({
        "schema_version": 1,
        "total_operations": operations.len(),
        "statuses": statuses,
        "rejections_by_reason": rejections,
        "override_statuses": override_targets
            .iter()
            .map(|(id, outcome)| (id.clone(), outcome["status"].clone()))
            .collect::<BTreeMap<_, _>>()
    });
    let baseline = read_json(&build.join("candidate-derivation-baseline.json"))?;
    if observed != baseline {
        println!("{}", serde_json::to_string_pretty(&observed)?);
        return fail(
            "candidate 288 derivation status/reason/override inventory drifted; review and update baseline only after proving the changed operations",
        );
    }

    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "candidate_derivation": {
                "statuses": statuses,
                "rejections_by_reason": rejections,
                "override_targets": override_targets,
                "report": "sdk-build/target/candidate-derivation-report.json"
            }
        }))?
    );
    Ok(())
}

struct Options {
    command: String,
    require_parity: bool,
    compatibility_definition: Option<PathBuf>,
}

fn parse_args() -> Result<Options> {
    let mut args = env::args().skip(1);
    let command = args.next().ok_or("usage: mistralai-sdk-build <raw|generate|check|probe|candidate-raw|candidate-derive> [--require-parity] [--compatibility-definition PATH]")?;
    if !matches!(
        command.as_str(),
        "raw" | "generate" | "check" | "probe" | "candidate-raw" | "candidate-derive"
    ) {
        return fail(format!("unknown command: {command}"));
    }
    let mut require_parity = false;
    let mut definition = None;
    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--require-parity" => require_parity = true,
            "--compatibility-definition" => {
                if definition.is_some() {
                    return fail("--compatibility-definition specified twice");
                }
                definition = Some(PathBuf::from(
                    args.next()
                        .ok_or("--compatibility-definition requires a path")?,
                ));
            }
            _ => return fail(format!("unknown argument: {flag}")),
        }
    }
    Ok(Options {
        command,
        require_parity,
        compatibility_definition: definition,
    })
}

fn execute(root: &Path, args: &Options) -> Result<()> {
    let lock = read_json(&root.join("sdk-build/provenance.lock.json"))?;
    let version = check_toolchain(root, &lock)?;
    sources::verify_sources(root, &lock)?;
    let raw = install_tool(root, &lock, "openapi_to_rust", "openapi-to-rust")?;
    let bindings = install_tool(
        root,
        &lock,
        "openapi_to_rust_bindings",
        "openapi-to-rust-bindings",
    )?;
    if args.command == "candidate-raw" {
        return verify_candidate_raw(root, &lock, &version, &raw, &bindings);
    }
    let compiler = install_tool(root, &lock, "rust_sdk_generator", "rust-sdk-generator")?;
    if args.command == "candidate-derive" {
        return verify_candidate_derivation(root, &lock, &version, &raw, &bindings, &compiler);
    }

    let temp = Workspace::new(root)?;
    let work = &temp.0;
    copy_inputs(root, work)?;
    let build = work.join("sdk-build");
    let published = build.join("openapi/published.yaml");
    let overlaid = build.join("openapi/overlaid.json");
    let config = build.join("openapi-to-rust.toml");
    let generated = work.join("src/generated");
    let raw_args = vec!["generate".into(), "--config".into(), str_arg(&config)];
    run(&raw, raw_args.clone(), root)?;
    let first = fs::read(&overlaid)?;
    let spec: Value = serde_json::from_slice(&first)?;
    verify_overlaid(&spec)?;
    run(&raw, raw_args.clone(), root)?;
    if fs::read(&overlaid)? != first {
        return fail("overlaid OpenAPI is not byte-for-byte deterministic");
    }
    let mut check = raw_args.clone();
    check.push("--check".into());
    run(&raw, check, root)?;
    let mut dry_run = raw_args;
    dry_run.push("--dry-run".into());
    run(&raw, dry_run, root)?;
    if fs::read(&published)? != fs::read(root.join("sdk-build/openapi/published.yaml"))? {
        return fail("raw generation modified the published OpenAPI");
    }
    format_rust(root, &version, &generated)?;
    let coverage = raw_coverage(&generated, &spec)?;
    write_json(&generated.join("coverage.json"), &coverage)?;

    if args.command == "raw" {
        // Source-update operation: raw generation is intentionally allowed to
        // change the committed raw baseline, but only after its own checks pass.
        let stage = work.join("raw-publication");
        copy_dir(&generated, &stage)?;
        stage_directory(
            &stage,
            &root.join("src/generated"),
            &work.join("raw-backup"),
        )?;
        println!("Regenerated raw OpenAPI bindings from pinned sources.");
        return Ok(());
    }
    normalized_coverage(root, &generated, &spec)?;
    let bindings_path = work.join("bindings.json");
    let bindings_value: Value = serde_json::from_str(&output(
        &bindings,
        vec![str_arg(&generated), str_arg(&overlaid)],
        root,
    )?)?;
    if bindings_value["schema_version"] != 3 {
        return fail("canonical bindings adapter did not emit Bindings v3");
    }
    write_json(&bindings_path, &bindings_value)?;

    let derivation: Value = serde_json::from_str(&output(
        &compiler,
        vec![
            "derive".into(),
            "--openapi".into(),
            str_arg(&overlaid),
            "--bindings".into(),
            str_arg(&bindings_path),
            "--surface".into(),
            str_arg(&build.join("official-sdks/surface.json")),
            "--overrides".into(),
            str_arg(&build.join("sdk-overrides.json")),
        ],
        root,
    )?)?;
    let definition = field(&derivation, "definition")?;
    let report = field(&derivation, "report")?;
    write_json(&work.join("sdk-definition.json"), definition)?;
    write_json(&work.join("derivation-report.json"), report)?;
    let baseline = read_json(&build.join("coverage-baseline.json"))?;
    let counts = validate_coverage(report, &baseline, &spec)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({"derivation_statuses": counts}))?
    );

    let runtime = json!({
        "error_type": "SdkError",
        "error_module": "error",
        "error_exports": ["ApiError", "SdkError", "TransportError", "TransportErrorKind"],
        "sse_module": "crate::streaming",
        "sse_function": "json_events",
        "generated_marker": "// @generated by rust-sdk-generator via sdk-build; do not edit by hand.\n"
    });
    write_json(&work.join("runtime.json"), &runtime)?;

    let generate = |definition: &Path, facade: &Path, inventory: &Path| -> Result<()> {
        run(
            &compiler,
            vec![
                "generate".into(),
                "--openapi".into(),
                str_arg(&overlaid),
                "--bindings".into(),
                str_arg(&bindings_path),
                "--definition".into(),
                str_arg(definition),
                "--runtime".into(),
                str_arg(&work.join("runtime.json")),
                "--output".into(),
                str_arg(facade),
                "--inventory".into(),
                str_arg(inventory),
            ],
            root,
        )?;
        format_rust(root, &version, facade)
    };

    let facade = work.join("src/sdk");
    let inventory = work.join("api-inventory.json");
    generate(&work.join("sdk-definition.json"), &facade, &inventory)?;
    let committed = committed_facade(&root.join("src/sdk"))?;
    let candidate_delta = compatible_snapshot(root, &facade, &committed)?;
    let mut final_delta = candidate_delta.clone();
    let definition_path = args.compatibility_definition.clone().or_else(|| {
        matches!(args.command.as_str(), "check" | "generate")
            .then(|| root.join("sdk-build/compatibility-definition.json"))
    });
    let mut compatible = None;
    if let Some(source) = definition_path {
        if !source.is_file() {
            return fail(format!(
                "compatibility SDK definition missing: {}",
                source.display()
            ));
        }
        let copy = work.join("compatibility-definition.json");
        fs::copy(source, &copy)?; // never normalize map insertion order in a reviewed definition
        let result = work.join("src/sdk-compatibility");
        generate(&copy, &result, &work.join("compatibility-inventory.json"))?;
        let delta = compatible_snapshot(root, &result, &committed)?;
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "canonical_candidate_delta_count": candidate_delta.len(),
                "compatibility_facade_delta_count": delta.len(),
                "compatibility_facade_delta": delta
            }))?
        );
        final_delta = delta;
        compatible = Some(result);
    }
    if args.require_parity || args.command == "check" || args.command == "generate" {
        require_publish_parity(&counts, &final_delta)?;
    }
    if args.command == "generate" {
        let compatible = compatible
            .as_ref()
            .ok_or("publishing requires compatibility SDK definition")?;
        publish(root, work, &generated, compatible)?;
    }
    let inventory = read_json(&inventory)?;
    let resources = field(&inventory, "resources")?
        .as_array()
        .ok_or("missing resources inventory")?;
    let models = field(&inventory, "models")?
        .as_array()
        .ok_or("missing models inventory")?;
    let slots: usize = resources
        .iter()
        .map(|resource| resource["operations"].as_array().map_or(0, Vec::len))
        .sum();
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "derivation_statuses": counts,
            "api_inventory": {
                "client": field(&inventory, "client")?,
                "models": models.len(),
                "resources": resources.len(),
                "operation_slots": slots
            },
            "facade_delta_count": final_delta.len(),
            "facade_delta": final_delta
        }))?
    );
    Ok(())
}

fn main() {
    let result = parse_args().and_then(|args| root().and_then(|root| execute(&root, &args)));
    if let Err(error) = result {
        eprintln!("sdk-build: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn candidate_voice_override_preserves_active_product_decisions() {
        let root = root().expect("repository root");
        let active =
            read_json(&root.join("sdk-build/sdk-overrides.json")).expect("active overrides");
        let candidate = read_json(&root.join("sdk-build/sdk-overrides-candidate.json"))
            .expect("candidate overrides");
        assert_eq!(candidate["schema_version"], active["schema_version"]);
        assert_eq!(
            candidate["excluded_operations"],
            active["excluded_operations"]
        );

        let active_operations = active["operations"].as_object().expect("active operations");
        let candidate_operations = candidate["operations"]
            .as_object()
            .expect("candidate operations");
        assert_eq!(candidate_operations.len(), active_operations.len() + 1);
        for (operation_id, selection) in active_operations {
            assert_eq!(candidate_operations.get(operation_id), Some(selection));
        }
        assert_eq!(
            candidate_operations
                .get("get_voice_sample_audio_v1_audio_voices__voice_id__sample_get"),
            Some(&serde_json::json!({
                "response_representations": {
                    "audio.voices.get_sample_audio": "binary_stream"
                }
            }))
        );
    }

    #[test]
    fn failed_second_publication_stage_restores_checkout() {
        let root = env::temp_dir().join(format!(
            "mistralai-sdk-build-rollback-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let work = root.join("work");
        let generated = work.join("generated");
        let compatible = work.join("compatible");
        let raw_target = root.join("src/generated");
        let sdk_target = root.join("src/sdk");
        for dir in [&work, &generated, &compatible, &raw_target, &sdk_target] {
            fs::create_dir_all(dir).unwrap();
        }
        fs::write(raw_target.join("client.rs"), b"previous raw").unwrap();
        fs::write(generated.join("client.rs"), b"candidate raw").unwrap();
        fs::write(sdk_target.join("client.rs"), b"previous facade").unwrap();
        fs::write(sdk_target.join("error.rs"), b"handwritten runtime").unwrap();
        fs::write(compatible.join("client.rs"), b"candidate facade").unwrap();

        // Force the SDK stage to fail after the raw stage has been installed.
        fs::create_dir(work.join("sdk-backup")).unwrap();
        assert!(publish(&root, &work, &generated, &compatible).is_err());
        assert_eq!(
            fs::read(raw_target.join("client.rs")).unwrap(),
            b"previous raw"
        );
        assert_eq!(
            fs::read(sdk_target.join("client.rs")).unwrap(),
            b"previous facade"
        );
        assert_eq!(
            fs::read(sdk_target.join("error.rs")).unwrap(),
            b"handwritten runtime"
        );
        fs::remove_dir_all(root).unwrap();
    }
}
