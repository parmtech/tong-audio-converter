# Development Roadmap

## Phase 0 — Architecture and governance
- [x] Public repository with Apache-2.0 license
- [x] Design API, failure types, codec strategy and integration goals
- [x] Public README, documentation and Wiki source
- [ ] Establish maintainer review policy and release governance

## Phase 1 — Working local core
- [ ] Implement Rust job coordinator and FFmpeg backend
- [ ] Probe, transcode and extract audio from permitted local files
- [ ] Publish actual MP3/WAV/FLAC/AAC/Opus codec-capability manifest
- [ ] Create a headless CLI; error handling, cancellation and output validation
- [ ] Unit / integration / corruption / low-disk / memory tests
- [ ] Validate Windows + macOS + Linux builds

## Phase 2 — Native mobile SDKs
- [ ] Android Kotlin/JNI wrappers, ABI targets and real-device tests
- [ ] iOS Swift/C wrappers and hardware tests
- [ ] CPU/memory/battery/storage benchmarks

## Phase 3 — Resilience and extensions
- [ ] Persistent queue, bounded retries and idempotency
- [ ] Metadata preservation, loudness and high-quality resampling
- [ ] Authenticated API worker with per-user quotas and SSRF prevention
- [ ] Optional authorized-source adapters independent of conversion

## Phase 4 — Open ecosystem
- [ ] Stable C ABI and language wrappers
- [ ] Plugin compatibility and conformance suite
- [ ] Portable builds with signed release artifacts and SBOM

Never mark a feature complete without tests and an accessible release or reproducible source proof.
