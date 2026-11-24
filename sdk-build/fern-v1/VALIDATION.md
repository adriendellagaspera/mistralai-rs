# Fern v1 validation scope

The candidate is generated from the same 347-operation OpenAPI source and
reviewed surface policy as the current production inventory.

- 347 HTTP operations, zero excluded.
- 349 expected public method paths: 347 operation methods plus the
  `chat.complete_stream` and `fim.complete_stream` aliases for conditional
  SSE responses.
- 1,770 public schema types in the checked-in surface inventory.
- Both `wrapped` and `inline` request-parameter styles are generated and
  compiled; their public function and query-type inventories must match.

Fern Rust v1.0.0 compatibility patches are applied before inventories and
compilation. They preserve the five one-value wire schemas as typed Rust enums
(`vespa`, `assistant`, `client_credentials`, `function`, and `code`);
correct multipart JSON/binary encoding; remove the incorrect Serde default on
a required enum; and restore the two conditional SSE methods. The patch tests
assert exact generator output and fail on drift.

The generated candidate CI validates formatting, compilation, Clippy and test
target execution. It does not yet replace the checked-in `src/` tree or prove
runtime parity. The existing transport contract tests and downstream examples
must be ported to the Fern v1 runtime before the production cutover can be
considered complete.
