# TONG MP3 Android source adapter

This headless source adapter uses youtubedl-android + FFmpeg to make verified local MP3s from authorized YouTube media. It has no UI and is not a standalone AAR yet. Copy to the Android host source tree and include youtubedl-android library + ffmpeg dependencies. The core conversion CLI remains platform-neutral; this adapter handles Android's extractor runtime only.

- Refreshes stable yt-dlp no more than daily in routine operation.
- Retries one SABR/format failure with the optional web_safari client; the upstream may still reject content.
- Validates actual audio MIME/duration before promoting into Audio Library.
- Explicit source error codes; private local temp files cleaned after each attempt.

**License:** Original code Apache-2.0. Third-party youtubedl-android is GPL-3.0. Shipping an APK with it may trigger copyleft/source obligations. Review before distribution. Only process authorized content.
