# openapi-to-rust-facade

`openapi-to-rust-facade` is a deterministic Rust facade compiler. Its core consumes three explicit, generator-independent inputs:

```text
OpenAPI -----------------------\
                                -> validated FacadeIr -> Rust facade
normalized Rust bindings IR ---/
explicit facade policy --------/
```

OpenAPI remains the wire-contract authority. The Rust bindings IR describes the concrete Rust types, callable operation signatures, symbol paths and raw-client binding that the facade must target. The policy supplies public naming and ergonomics that HTTP cannot express. Ambiguous drift fails closed.

The compiler does **not** require or import `openapi-to-rust`. `openapi-to-rust` is one optional backend adapter that normalizes its generated `types.rs` / `client.rs` into the same bindings IR.

## Core API

Install the compiler only:

```bash
pip install openapi-to-rust-facade
```

```python
from openapi_to_rust_facade import OpenApiIndex, RawIr, compile_facade

raw = RawIr.from_dict(bindings_sidecar)
ir, files = compile_facade(OpenApiIndex(document), raw, policy)
```

`raw-ir.schema.json` is the versioned machine-readable contract. Version 2 carries full Rust symbol paths plus an explicit raw-client binding, so the compiler contains no generated-module or client-name convention.

Facade runtime support is a separate explicit input. `RustFacadeRuntime` controls the stable error module/type, SSE helper and generated marker without coupling those concerns to the raw-binding backend.

## openapi-to-rust adapter

Install the optional adapter only when source normalization is needed:

```bash
pip install 'openapi-to-rust-facade[openapi-to-rust]'
```

```python
from openapi_to_rust_facade.adapters.openapi_to_rust import OpenApiToRustAdapter

raw = OpenApiToRustAdapter.parse(types_source, client_source)
```

The adapter is intentionally the only component that knows `openapi-to-rust` source layout, `HttpClient`, generated module paths or tree-sitter. If `openapi-to-rust` eventually emits the bindings sidecar directly, this adapter can collapse to sidecar loading without changing the compiler.

## Compatibility

Version `0.2.0` retains a tested adapter for `openapi-to-rust` `0.16.0`, repository `gpu-cli/openapi-to-rust`, commit `2af34b86ca9f38c35787f13ec5841989efcf4b99`. `COMPATIBILITY.json` records the exact backend revision and Rust bindings IR schema version.

Backend compatibility and compiler compatibility are deliberately separate: advancing `openapi-to-rust` is acceptable when its normalized bindings still compile to the same facade semantics.

## Validation

The CI first installs the compiler **without** tree-sitter and compiles Menagerie and Library from checked-in bindings sidecars. It then installs the `openapi-to-rust` extra and verifies that parsing the backend fixtures reproduces those sidecars exactly.

Menagerie exercises discriminated request unions and wrapper projection. Library exercises nested resources, JSON bodies, optional parameters, empty success responses and binary streams. Focused tests also verify scalar enums, deterministic generation, configurable runtime support and the absence of backend layout constants from compiler-core modules.

## Scope

The compiler owns generic Rust-facade concerns only:

- OpenAPI/wire indexing;
- normalized Rust bindings IR;
- declarative facade policy and validation;
- lowering to resolved `FacadeIr`;
- deterministic Rust emission;
- generic fixtures and contracts.

Product taxonomy discovery, API coverage policy, source-SDK reconciliation and repository-specific release checks belong to consumers such as `mistralai-rs`.

## Provenance and license

The compiler sources are extracted from the same repository history that introduced the facade compiler. Compatible raw-generator backends are referenced and tested but their source code is not vendored. The package is licensed under either MIT or Apache-2.0, at your option, preserving a clean path for future upstream contribution.
