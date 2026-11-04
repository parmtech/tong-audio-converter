# Third-Party Research — Feature Inspiration, Not Copying

Reviewed November 2026. This is an architectural comparison; current releases/security status must be rechecked before importing dependencies.

| Project | Repository | Valuable capability | Integration policy |
|---|---|---|---|
| FFmpeg | https://github.com/FFmpeg/FFmpeg | Broad codecs, transcode, filters, ffprobe | Core backend; explicit licensing matrix |
| GStreamer | https://github.com/GStreamer/gstreamer | Plugin pipeline, cross-platform native acceleration | Architectural inspiration / optional adapter; avoid mandatory extra runtime |
| yt-dlp | https://github.com/yt-dlp/yt-dlp | Provider extraction, metadata, diagnostics | Optional external source adapter; upstream failures must not break core |
| Symphonia | https://github.com/pdeljanov/symphonia | Safe Rust decoder, probing and tags | Evaluate as lightweight optional probe/decoder |
| Rubato | https://github.com/HEnquist/rubato | High-quality Rust resampling | Evaluate DSP integration after benchmarks |
| libsamplerate | https://github.com/libsndfile/libsamplerate | Mature sampling rate conversion | Comparison baseline / optional backend |
| RustAudio/dasp | https://github.com/RustAudio/dasp | Audio DSP primitives | Lightweight manipulation layer |
| FFmpegKit / FFmpegKitNext | https://github.com/arthenica/ffmpeg-kit | Mobile build/wrapper patterns | Old FFmpegKit retired; research maintained continuation before use |
| BtbN FFmpeg Builds | https://github.com/BtbN/FFmpeg-Builds | Reproducible build variants | Build pipeline inspiration only; do not blindly redistribute binaries |

## Lessons

**FFmpeg** is the practical conversion workhorse. Re-implementing all codecs would be a security and maintenance mistake. **Rust** is a good orchestration and FFI boundary, but portable source code does not mean identical binary artifacts across platforms. **GStreamer** demonstrates that pluggable graphs are powerful but should not be required for simple MP3 jobs. **yt-dlp** is an extractor, not an encoder; provider formats can change (e.g. SABR), so it must not become the core converter.

## Decision records

- ADR-001: Rust orchestration, language-neutral C ABI, platform-native wrappers.
- ADR-002: Decoder/encoder capability reported per build, no universal support promise.
- ADR-003: Local media conversion works with no Internet connection.
- ADR-004: External source adapters opt-in and isolated from codec layer.
- ADR-005: Avoid retired FFmpegKit dependency as an unreviewed foundation.
- ADR-006: Preserve original file on failures; write output atomically.
- ADR-007: Original project Apache-2.0; preserve third-party license obligations.