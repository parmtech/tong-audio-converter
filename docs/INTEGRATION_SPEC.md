# Public Integration Contract — Proposed, not implemented

TONG Audio Engine is a **headless** media component designed for native apps, command-line automation and optionally authenticated workers.

## Operations

| Method | Output |
|---|---|
| `capabilities()` | Encoders, decoders, muxers, build flags, restrictions |
| `probe(source)` | Streams, duration, tags, format, errors |
| `submit(job_spec)` | Stable job ID |
| `status(job_id)` | State, phase, time/byte progress, structured errors |
| `cancel(job_id)` | Stop and clean temporary output |
| `result(job_id)` | Verified output file, checksum, stream metadata |

### Example proposed job JSON (NOT live API)

```json
{
  "schema_version": 1,
  "source": { "kind": "local_file", "path": "./voice.wav" },
  "output": { "path": "./voice.mp3", "format": "mp3", "bitrate_kbps": 192 },
  "policy": { "overwrite": false }
}
```

## Integration targets

- **Android:** Rust-generated native libraries, Kotlin/JNI wrapper, background work policies.
- **iOS:** C ABI/Swift wrapper, bundled app-signed native media dependencies.
- **Windows:** DLL and/or CLI EXE with compatible architecture.
- **macOS:** Framework or CLI, notarization as needed.
- **Linux:** Shared library or CLI package.
- **Servers:** Optional restricted authenticated REST worker. A shared hosting account may prohibit running conversion subprocesses.

**Portability means a common contract and equivalent behavior—not a single executable file for every platform.** No GUI is part of this repository.
