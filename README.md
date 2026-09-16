# rust-sdk-compiler

`rust-sdk-compiler` deterministically compiles an idiomatic Rust SDK surface from four explicit inputs:

- `OpenApi`: the HTTP/wire contract;
- `Bindings`: the Rust types and callable operations that actually exist;
- `Policy`: the public SDK shape to expose;
- `Runtime`: consumer-owned support conventions such as error and streaming paths.

The compiler is backend-neutral. It does not parse generator output, know `openapi-to-rust`, or depend on tree-sitter. Generator-specific compatibility packages only need to produce `Bindings`.

```python
from rust_sdk_compiler import Bindings, OpenApi, Policy, Runtime, compile

openapi = OpenApi.load("openapi.yaml")
bindings = Bindings.from_dict(...)
policy = Policy.from_dict(...)

compilation = compile(openapi, bindings, policy, runtime=Runtime())
compilation.ir
compilation.files
```

`lower(openapi, bindings, policy)` stops after semantic lowering and returns the resolved SDK IR without rendering Rust files.

The versioned `rust-bindings.schema.json` sidecar is the stable interchange contract for backend-specific producers. The compiler fails closed on unsupported or ambiguous source shapes.
