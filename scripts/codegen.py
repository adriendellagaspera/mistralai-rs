"""Pinned, isolated code generation. Python 3.11+ with venv and Git."""

import argparse
import difflib
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import tomllib
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))
import preprocess


def run(*args, cwd=ROOT):
    subprocess.run([str(arg) for arg in args], cwd=cwd, check=True)


def verify_spec(data, lock):
    actual = hashlib.sha256(data).hexdigest()
    if actual != lock["spec_sha256"]:
        raise ValueError(f"Spec SHA-256 mismatch: expected {lock['spec_sha256']}, got {actual}")


def snapshot(directory):
    return {p.relative_to(directory).as_posix(): p.read_bytes()
            for p in sorted(directory.rglob("*")) if p.is_file()}


def differences(expected, actual):
    return sorted(k for k in expected.keys() | actual.keys()
                  if expected.get(k) != actual.get(k))


def generator(lock):
    if lock["generator"] != "openapi-to-rust":
        raise ValueError("Unsupported generator")
    version = lock["generator_version"]
    patch = ROOT / "codegen/patches/generator.patch"
    digest = hashlib.sha256(patch.read_bytes()).hexdigest()
    if digest != lock["generator_patch_sha256"]:
        raise ValueError("Generator patch SHA-256 mismatch")
    install = ROOT / ".tools" / f"openapi-to-rust-{lock['generator_commit']}-{digest}"
    executable = install / "bin" / "openapi-to-rust"
    if not executable.exists():
        with tempfile.TemporaryDirectory(prefix="generator-") as temporary:
            source = Path(temporary)
            run("git", "init", source)
            run("git", "fetch", "--depth=1",
                f"https://github.com/{lock['generator_repository']}.git",
                lock["generator_commit"], cwd=source)
            run("git", "checkout", "--detach", "FETCH_HEAD", cwd=source)
            run("git", "apply", "--check", patch, cwd=source)
            run("git", "apply", patch, cwd=source)
            run("cargo", f"+{lock['rust_toolchain']}", "install", "--locked",
                "--path", source, "--root", install)
    output = subprocess.check_output([str(executable), "--version"], text=True).strip()
    if output != f"openapi-to-rust {version}":
        raise ValueError(f"Unexpected generator: {output}")
    return executable


def tooling_python(lock):
    environment = ROOT / ".tools" / f"python-ruamel-yaml-{lock['ruamel_yaml_version']}"
    python = environment / "bin/python"
    if not python.exists():
        run(sys.executable, "-m", "venv", environment)
    try:
        version = subprocess.check_output(
            [str(python), "-c", "import ruamel.yaml; print(ruamel.yaml.__version__)"], text=True).strip()
    except subprocess.CalledProcessError:
        version = None
    if version != lock["ruamel_yaml_version"]:
        run(python, "-m", "pip", "install", f"ruamel.yaml=={lock['ruamel_yaml_version']}")
    return python


def apply_layout(generated, lock):
    manifest = ROOT / "codegen/layout/Cargo.toml"
    lockfile = ROOT / "codegen/layout/Cargo.lock"
    if not lockfile.exists():
        raise ValueError("Missing codegen/layout/Cargo.lock; generate and commit the layout tool lockfile")
    run("cargo", f"+{lock['rust_toolchain']}", "run", "--locked", "--quiet",
        "--manifest-path", manifest, "--", generated)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["generate", "check"])
    args = parser.parse_args()
    lock = json.loads((ROOT / "codegen.lock").read_text())
    toolchain = tomllib.loads((ROOT / "rust-toolchain.toml").read_text())
    if toolchain["toolchain"]["channel"] != lock["rust_toolchain"]:
        raise ValueError("rust-toolchain.toml and codegen.lock disagree")
    verify_spec((ROOT / "spec/openapi.yaml").read_bytes(), lock)
    executable = generator(lock)
    python = tooling_python(lock)
    # Preserve relative paths from the checked-in config; never modify its options.
    with tempfile.TemporaryDirectory(prefix=".codegen-", dir=ROOT) as temp:
        work = Path(temp)
        shutil.copytree(ROOT / "spec", work / "spec")
        shutil.copytree(ROOT / "codegen", work / "codegen")
        run(python, work / "codegen/preprocess.py",
            work / "spec/openapi.yaml", work / "spec/openapi.codegen.yaml")
        config = work / lock["generator_config"]
        run(executable, "generate", "--config", config)
        # The generator's own check validates pristine generator output first.
        run(executable, "generate", "--config", config, "--check")
        generated = work / "src/generated"
        # Coverage must inspect the generator's complete monolithic output before
        # the physical layout step rewrites it into include files.
        run(python, ROOT / "codegen/coverage.py", work / "spec/openapi.yaml",
            work / "spec/openapi.codegen.yaml", generated)
        apply_layout(generated, lock)
        for path in sorted(generated.rglob("*.rs")):
            run("rustup", "run", lock["rust_toolchain"], "rustfmt",
                "--edition", "2024", "--config", "skip_children=true", path)
        target = ROOT / "src/generated"
        if args.command == "generate":
            if target.exists():
                shutil.rmtree(target)
            shutil.copytree(generated, target)
            print("Generated SDK from verified, pinned spec with deterministic split layout.")
        else:
            old, new = snapshot(target), snapshot(generated)
            changed = differences(old, new)
            for name in changed:
                print("".join(difflib.unified_diff(
                    old.get(name, b"").decode().splitlines(keepends=True),
                    new.get(name, b"").decode().splitlines(keepends=True),
                    fromfile=f"committed/{name}", tofile=f"regenerated/{name}")))
            if changed:
                raise SystemExit("Generated SDK is stale: " + ", ".join(changed))
            print("Generated SDK matches byte-for-byte (including split file set).")


if __name__ == "__main__":
    main()
