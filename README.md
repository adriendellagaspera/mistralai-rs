# openapi-to-rust-bindings

`openapi-to-rust-bindings` is the compatibility layer between `openapi-to-rust` generated Rust and `rust-sdk-compiler`.

It exposes a deliberately small functional API:

```python
from openapi_to_rust_bindings import parse_bindings, read_bindings

bindings = read_bindings(generated_dir)
# or
bindings = parse_bindings(types_source, client_source)
```

Both functions return the public `Bindings` type from `rust-sdk-compiler`. This package owns all knowledge of `openapi-to-rust` source layout and conventions (`types.rs`, `client.rs`, `HttpClient`, generated module paths and BoxStream encoding). It does not import compiler internals.

The package is validated against the exact `openapi-to-rust` backend revision recorded in `COMPATIBILITY.json` and against an exact `rust-sdk-compiler` commit.
