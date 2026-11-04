# Architecture — TONG Audio Engine

**Design goals:** no GUI; small reusable interfaces; platform-specific implementation details hidden; offline processing supported; explicit limits; auditable distribution.

## Component boundaries

1. **API layer (`tong_audio`)** — versioned public methods and compatibility semantics, input/output schemas, stable identifiers.
2. **Job supervisor** — queued/running/succeeded/failed/cancelled states, persistent recovery where configured, bounded concurrency and cancellation.
3. **Source adapters** — local filesystem, application-provided media stream, authorized direct HTTP file; optional third-party provider adapters out-of-tree.
4. **Media probe** — detect container, tracks, codec, duration, seekability, sample rate, channel layout, tags and format compatibility. Do not trust file extension alone.
5. **Codec backends** — FFmpeg (native library where required, CLI when feasible), with explicit per-build supported-encoder/decoder manifest.
6. **Audio DSP** — normalization, trim, channel mapping, resampling and optional gapless-aware processing.
7. **Output writer** — safe target path, temporary file, fsync where applicable, atomic rename, checksum and validation.
8. **Telemetry** — local structured logs, redaction of secrets and URLs, consistent machine-readable error codes; no mandatory cloud collection.

## Public API — illustrative proposal (not implemented)

```json
{
  "job_id": "job_01",
  "source": {"kind": "local_file", "path": "/media/input.wav"},
  "output": {"path": "/media/output.mp3", "format": "mp3", "bitrate_kbps": 192, "sample_rate_hz": 44100},
  "processing": {"normalize_loudness": false},
  "policy": {"overwrite": false, "max_parallel_jobs": 1}
}
```

```json
{"job_id":"job_01","state":"running","phase":"encoding","progress_percent":43,"error":null}
```

Every capability and field will carry a schema version. Until an actual release exists these examples are **design contracts, not callable production endpoints**.

## Error taxonomy

| Code | Meaning | Automatic action |
|---|---|---|
| `INPUT_NOT_FOUND` | File missing | Do not retry |
| `UNSUPPORTED_INPUT` | No supported demuxer/decoder | Offer capability report |
| `CODEC_UNAVAILABLE` | Encoder unavailable in build | Report build variant |
| `SOURCE_ACCESS_DENIED` | Source is inaccessible/not authorized | Do not bypass restrictions |
| `SOURCE_FORMAT_UNAVAILABLE` | Upstream cannot expose a usable stream | Source adapter diagnostic |
| `SOURCE_UPSTREAM_CHANGED` | E.g. upstream streaming protocol changed | Safe retry only after adapter update |
| `NETWORK_INTERRUPTED` | Network download interrupted | Bounded, resumable retry when supported |
| `INSUFFICIENT_STORAGE` | Insufficient storage | Retain input, clean temporary output |
| `OUTPUT_INVALID` | Validation/hash/stream probe failed | Do not publish output |
| `JOB_CANCELLED` | User/application cancelled job | Stop and clean temp output |

## Platform implementation

| OS | Planned integration | Constraints |
|---|---|---|
| Android | Rust `.so`, Kotlin/JNI SDK | ABI, background limits, mobile memory, package size |
| iOS | Rust static/dynamic lib + Swift/C wrapper | App Store policies, code signing and bundled native libs |
| Windows | DLL and/or CLI EXE | Signed releases recommended, file path / process security |
| macOS | `.dylib`/framework or CLI | Notarization, hardened runtime as needed |
| Linux | `.so`/CLI | ABI and distro compatibility |
| Backend | Headless CLI and optional REST worker | Resource quotas, sandboxing, no unauthenticated fetch |

## Streaming, memory and reliability

Use chunked or streaming I/O where possible, not full-file buffers. Set configurable job memory/CPU/time/disk limits and test low-memory devices. For shared hosting, do not assume long-running FFmpeg workers or subprocesses are permitted; local conversion or a dedicated worker may be necessary.

Implement rate-limited progress events; checkpoint jobs to avoid duplicate completed outputs; idempotency keys for network/API submissions. Keep source media unless policy explicitly allows removal and output passes independent probe.

## Security model

- Any URL-capable server worker must defend against SSRF: allowlisted schemes, DNS and redirect revalidation, no private/local address fetches and bounded download sizes.
- No arbitrary shell-constructed commands. Use argument arrays or linked library APIs.
- Never log cookies, OAuth values, bearer tokens or unredacted private URLs.
- Limit file paths to configured workspaces, prevent path traversal and symlink attacks.
- Require auth, quotas and per-user isolation for remote workers.
- Generate signed release artifacts, SBOM and notices; run fuzzing against media parsing interfaces.

## Performance / quality acceptance

Baseline per platform: 1-, 10- and 60-minute samples, multiple bitrates and codec combinations; Android low-memory test and cross-ABI tests. Measure peak RSS, CPU, duration, battery use, output file size, playback validation, crash recovery, cancellation correctness. Publish benchmarks with device/OS/build versions—not universal speed promises.