# Quality, Reliability and Security Gates

- Never read an entire large input file into memory when streaming is possible.
- Detect actual formats using probes, not filename extensions.
- Strict path confinement for server use, prevent symlink and traversal attacks.
- Never construct shell command strings from filenames; pass explicit argument arrays.
- Cap CPU, memory, files, output sizes, job times and concurrent work.
- Atomic output publication: write temporary file, validate it and rename safely.
- Preserve input on failure; checksum and independent media probe on completed outputs.
- Sanitize diagnostic logs; no credentials, cookies or signed URLs.
- URL-capable worker: validate schemes, redirects and resolved IPs to prevent SSRF.
- Exercise fuzzing and malformed input tests; publish per-platform performance details.
- Document licensing and runtime codec manifests for every distributed artifact.
