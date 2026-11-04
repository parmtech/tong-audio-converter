# Feature Matrix

**Status:** Planned. No SDK, CLI or conversion binary is released yet.

| Capability | Milestone | Definition of done |
|---|---|---|
| Probe MP3, WAV, FLAC, AAC, Opus | MVP | Real files, verified stream metadata and clean errors |
| Convert audio and extract authorized video audio streams | MVP | Output tested independently with ffprobe |
| MP3/WAV/FLAC/AAC/Opus output | MVP | Per-build actual codec manifest, golden fixtures |
| Bitrate, sample rate, channel conversion | MVP | Parameter validation and deterministic reports |
| Cancellation, progress, temp outputs | MVP | Cancel / disk full / interruption tests |
| Offline CLI, C ABI | MVP | Windows, macOS and Linux runners |
| Android integration (JNI) | Phase 2 | arm64 and x86_64 device tests |
| iOS integration (Swift) | Phase 2 | Device test + app policy check |
| Persistent queue, loudness, metadata processing | Phase 3 | Crash recovery and quality test |
| Authenticated backend worker | Phase 3 | Auth, quotas, SSRF mitigations, resource budgets |
| Provider URL source adapters | Later | Explicit opt-in, lawful access, decoupled failure policy |

Do not advertise universal codec support: publish machine-readable capabilities for every actual build. Lossy sources never gain quality when transcoded to a lossless container.
