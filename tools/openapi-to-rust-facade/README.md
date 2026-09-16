# openapi-to-rust-facade

`openapi-to-rust-facade` compiles a deterministic, idiomatic Rust facade from three explicit inputs:

```text
OpenAPI --------------------------\
                                   -> validated FacadeIr -> Rust facade
openapi-to-rust output -> RawIr --/
explicit facade policy ----------/
```

OpenAPI remains the wire-contract authority. `RawIr` describes the generated Rust symbols and signatures the facade must call. The policy supplies public naming and ergonomics that HTTP cannot express. Ambiguous drift fails closed.

## Compatibility

Version `0.1.0` is validated against `openapi-to-rust` `0.16.0`, repository `gpu-cli/openapi-to-rust`, commit `2af34b86ca9f38c35787f13ec5841989efcf4b99`. The exact machine-readable contract is also recorded in `COMPATIBILITY.json`.

The adapter currently parses `types.rs` and `client.rs`. `RawIr` is serializable through `to_dict()` / `from_dict()`, and `raw-ir.schema.json` defines the candidate sidecar an upstream generator could emit to remove source reparsing later.

## API

```python
from openapi_to_rust_facade import OpenApiIndex, OpenApiToRustAdapter, compile_facade

openapi = OpenApiIndex(document)
raw = OpenApiToRustAdapter.parse(types_source, client_source)
ir, files = compile_facade(openapi, raw, policy)
```

The compiler does not discover product taxonomy, track API coverage, or perform repository-specific semver audits. Those are consumer concerns.

## Validation

The standalone test suite contains two unrelated fixtures. Menagerie exercises discriminated request unions and wrapper projection. Library exercises nested resources, JSON bodies, optional parameters, empty success responses, and binary success streams. Running the same fixture twice must produce byte-identical source maps.

## Provenance and license

The compiler sources in this package are extracted from the same repository history that introduced the facade compiler; the compatible `openapi-to-rust` backend is referenced and tested but none of its source code is vendored here. The package is licensed under either MIT or Apache-2.0, at your option, matching the included license files and keeping later upstream contribution possible.
