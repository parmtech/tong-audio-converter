# License and Distribution Compliance

**Repository license: Apache License 2.0**, as defined in the root [LICENSE](../LICENSE).

TONG's own files may be distributed under Apache-2.0, **but third-party components keep their own licenses**.

| Dependency | License implications |
|---|---|
| FFmpeg | LGPL and GPL variants; compile flags and enabled dependencies determine obligations |
| GStreamer | Framework LGPL, individual plugin/dependency terms vary |
| Symphonia | MPL-2.0; preserve its file-level conditions |
| yt-dlp | Main code public domain/Unlicense; optional dependencies vary |
| Rubato | MIT/Apache-2.0 options; verify exact version |
| libsamplerate | BSD license; retain notices |

Before shipping binaries: enumerate every dependency and codec, capture SPDX metadata and hashes, produce SBOM and notices, avoid non-redistributable builds, verify obligations for distribution/relinking where relevant, and document each release's enabled features and platform restrictions.

Licensing of codecs, patents and media rights can be jurisdiction-specific. Open source does not grant unrestricted media downloading rights. This document is engineering guidance, not legal advice.

References: [FFmpeg](https://github.com/FFmpeg/FFmpeg/blob/master/LICENSE.md), [GStreamer](https://gstreamer.freedesktop.org/documentation/frequently-asked-questions/licensing.html).
