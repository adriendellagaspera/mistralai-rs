# Scheduled Mistral upstream reconciliation

The README has two **run-status** badges: daily source/SDK check and weekly
independent review. They deliberately turn red when a change is detected or
upstream checks fail; red is not synonymous with a broken Rust build.

Both schedules run the same independently sampled pipeline:

1. Download and parse the deployed Mistral OpenAPI catalog (not a byte mirror
   of the historical platform-docs-public source).
2. Query the official Python and TypeScript SDK heads independently.
3. Compare every observed revision with the pinned source and SDK revisions.
4. Prepare a candidate with the new OpenAPI input and SDK pins on one stable
   automation branch; attempt complete Fern regeneration, Rust tests and
   publication checks.
5. Create or refresh one PR with an explicit compatibility and acquisition
   report. Even if generation fails, **the remediation PR still opens**.
6. Fail the scheduled workflow while any divergence or source failure remains.
   Its badge becomes green after a reviewed candidate lands on main and the
   following run confirms upstream is still unchanged.

The OpenAPI source migration is explicit in the generated PR:
\`source_kind=published_catalog\` and \`url=https://docs.mistral.ai/openapi.yaml\`.
The historical GitHub source disappeared and cannot silently be assumed
equivalent to the larger published spec. A candidate may include breaking
API changes and MUST NOT be merged without checking code generation,
license provenance, the SDK surface and downstream users.

The report in \`STATUS.md\` is generated **only on the automation branch**.
It is not committed to main by routine scans. The README's GitHub Actions
badges are live and intentionally avoid promising binary/runtime compatibility.

For reproducible diagnosis, dispatch either workflow manually from GitHub
Actions. The automation branch is \`automation/update-mistral-sources\`;
the PR is reused, not multiplied. If GitHub Actions does not have
"Read and write permissions" / "Allow GitHub Actions to create and approve
pull requests" enabled, publishing the candidate will fail visibly.

## GitHub approval is the only human acceptance record

CI validates source identity, generator reproducibility, coverage, and Rust tests.
The protected `main` ruleset separately requires one human PR approval,
last-push approval, and successful `gate` CI. An OpenAPI source pin is a
**proposal**, not an approval. Never set `PENDING_PR_REVIEW` or a synthetic
review date in a lockfile: GitHub tracks the actual approval and reviewer.
Review the semantic diff and source provenance, then select **Review changes →
Approve** in the pull request. Any subsequent push invalidates the approval.
