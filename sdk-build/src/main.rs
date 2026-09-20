mod gates;

use gates::{
    Result, committed_facade, copy_dir, facade_delta, fail, field, raw_coverage, read_json,
    require_publish_parity, snapshot, string, validate_coverage, verify_overlaid,
    verify_raw_baseline, verify_raw_coverage, write_json,
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

fn verify_sources(root: &Path, lock: &Value) -> Result<()> {
    let published = root.join("sdk-build/openapi/published.yaml");
    if !published.is_file() {
        return fail("pinned published OpenAPI is missing");
    }
    // The consumer-owned source checker retains its dialect/overlay assumptions
    // and independently verifies the pinned SHA-256, without fetching an update.
    if field(lock, "openapi")?
        .get("sha256")
        .and_then(Value::as_str)
        .is_none()
    {
        return fail("OpenAPI SHA-256 pin is missing");
    }
    run(
        "python3",
        vec![str_arg(&root.join("sdk-build/openapi/check_published.py"))],
        root,
    )
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
        "coverage-baseline.json",
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

struct Options {
    command: String,
    require_parity: bool,
    compatibility_definition: Option<PathBuf>,
}

fn parse_args() -> Result<Options> {
    let mut args = env::args().skip(1);
    let command = args.next().ok_or("usage: mistralai-sdk-build <raw|generate|check|probe> [--require-parity] [--compatibility-definition PATH]")?;
    if !matches!(command.as_str(), "raw" | "generate" | "check" | "probe") {
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
    verify_sources(root, &lock)?;
    let raw = install_tool(root, &lock, "openapi_to_rust", "openapi-to-rust")?;
    let compiler = install_tool(root, &lock, "rust_sdk_generator", "rust-sdk-generator")?;
    let bindings = install_tool(
        root,
        &lock,
        "openapi_to_rust_bindings",
        "openapi-to-rust-bindings",
    )?;

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
    let bindings_value: Value =
        serde_json::from_str(&output(&bindings, vec![str_arg(&generated)], root)?)?;
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
