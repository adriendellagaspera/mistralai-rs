# openapi-rust-facade

`openapi-rust-facade` is a deterministic compiler for idiomatic Rust facades. Its core has no dependency on a particular Rust OpenAPI generator.

```text
OpenAPI -------------------\
RustBindingsIr -------------> validated FacadeIr -> deterministic Rust facade
explicit facade policy ----/
RustFacadeRuntime ---------/
```

OpenAPI is the wire-contract authority. `RustBindingsIr` describes the Rust types, operation signatures, symbol paths and raw-client binding that already exist. The explicit facade policy supplies public naming and ergonomics that HTTP cannot express. Ambiguous drift fails closed.

## Compiler API

The core package depends only on YAML and JSON-schema support. It does not import tree-sitter or parse generated Rust source.

```python
from openapi_rust_facade import OpenApiIndex, RustBindingsIr, compile_facade

wire = OpenApiIndex(openapi_document)
bindings = RustBindingsIr.from_dict(bindings_document)
ir, files = compile_facade(wire, bindings, policy)
```

`RustBindingsIr` schema version 2 is defined by `rust-bindings.schema.json`. Generator-specific module layouts and client names are data in that contract, not compiler constants. Consumer-owned error and streaming support is supplied through `RustFacadeRuntime`.

## openapi-to-rust adapter

`openapi-to-rust` is one optional bindings backend, not part of the compiler core. Install the adapter dependencies with:

```text
pip install 'openapi-rust-facade[openapi-to-rust]'
```

Then normalize generated source into the generic bindings contract:

```python
from openapi_rust_facade.adapters.openapi_to_rust import OpenApiToRustAdapter

bindings = OpenApiToRustAdapter.load(generated_directory)
```

Only this adapter knows the `types.rs` / `client.rs` layout, `HttpClient`, `crate::generated::*`, serde attributes, or the Rust AST parser. An upstream machine-readable bindings sidecar can eventually replace this source parsing without changing the compiler.

## Compatibility

Version `0.2.0` of the compiler uses Rust bindings schema version 2. The bundled `openapi-to-rust` adapter is validated against `openapi-to-rust` `0.16.0`, repository `gpu-cli/openapi-to-rust`, commit `2af34b86ca9f38c35787f13ec5841989efcf4b99`. The exact contract is recorded in `COMPATIBILITY.json`.

Compatibility is intentionally split in two:

- compiler compatibility is defined by OpenAPI + `RustBindingsIr` + facade policy + runtime contract;
- adapter compatibility is defined by whether a concrete generator revision can still normalize its output into equivalent `RustBindingsIr` and preserve the resulting facade.

## Validation

The standalone suite contains two unrelated fixtures. Menagerie exercises discriminated request unions and wrapper projection. Library exercises nested resources, JSON bodies, optional parameters, empty responses and binary streams. Core tests compile only from checked-in `RustBindingsIr` sidecars; adapter tests separately prove that `openapi-to-rust` source parsing reproduces those sidecars exactly. Repeated compilation must produce byte-identical source maps.

## Provenance and license

The compiler was extracted from the facade work originally developed in `mistralai-rs`; no `openapi-to-rust` source code is vendored. The package is licensed under either MIT or Apache-2.0, at your option, preserving a clean path for later upstream contribution.
