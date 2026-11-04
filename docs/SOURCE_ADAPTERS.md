# Source Adapters and External Media URLs

The core engine converts media **already accessible** to the application. Acquisition is a separate component.

1. **Local files:** highest-priority first-class input; no network necessary.
2. **Application-owned streams:** pass an authorized byte stream or file handle.
3. **Direct URLs:** later optional connector with host restrictions and explicit network policy.
4. **Provider extractors (e.g. yt-dlp):** separately versioned optional modules only, subject to rights and platform restrictions.

## Handling SABR and missing stream URLs

Some streaming sites change their client responses or transport technology. An extractor may then return a failure such as `SOURCE_FORMAT_UNAVAILABLE` or `SOURCE_UPSTREAM_CHANGED`. An available FFmpeg encoder cannot fix a missing input stream. Log clear source diagnostics (without secrets), keep the local conversion pipeline functional, and allow plugins to update independently.

No DRM bypass or guarantee of downloading restricted content. Do not ship user authentication cookies inside app binaries.
