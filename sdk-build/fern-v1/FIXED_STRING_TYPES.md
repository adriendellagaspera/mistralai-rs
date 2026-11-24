# Fixed-string schema compatibility

Fern Rust generator 1.0.0 currently references these schema types from generated
models but omits their declarations. The compatibility step emits one-variant
Rust enums, not unrestricted `String` aliases, so serialization remains
constrained to the OpenAPI wire value.

| Rust type | Variant | Wire value |
|---|---|---|
| `GetDeploymentSummariesResponseVespaDeploymentType` | `Vespa` | `vespa` |
| `AssistantMessageRole` | `Assistant` | `assistant` |
| `OAuth2ClientCredentialsAuthMethodGrantType` | `ClientCredentials` | `client_credentials` |
| `FunctionToolType` | `Function` | `function` |
| `OcrCodeBlockType` | `Code` | `code` |

The enums derive `Hash` as well as Serde, equality, and copy traits because
several containing generated models also derive `Hash`. The patch validates
all five generated type references and fails if the expected counts or
generator output change.
