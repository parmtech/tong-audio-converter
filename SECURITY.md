# Security Policy

TONG Audio Engine is in architecture/planning. Production vulnerability-reporting contacts and supported versions will be added before its first release.

Do not share user secrets, access cookies, signed media URLs, authentication tokens or personal media in public GitHub issues. Until a private disclosure process is established, open public issues only for non-sensitive problems.

## Planned security baseline

- Process untrusted media in a constrained environment with fuzz-tested parsing surfaces.
- Bound CPU, RAM, runtime, I/O and disk for every conversion job.
- Protect URL workers against SSRF, redirect bypass, DNS rebinding and oversized media.
- Validate path and file names; use safe temporary paths and atomic writes.
- Redact credentials from structured logs; keep third-party dependency tracking and SBOMs.
- Avoid shell injection by using native APIs or argument arrays.