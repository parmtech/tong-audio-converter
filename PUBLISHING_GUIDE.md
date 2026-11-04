# Maintenance and Publishing Guide

The repository is public under **Apache License 2.0**.

1. Review README, architecture and planned feature matrix before implementing capabilities.
2. Public release requirements: actual tests, deterministic builds, license notices, SBOM, artifacts, hashes, changelog and clear supported-codec manifest.
3. GitHub's **Wiki tab is not populated by the `wiki/` directory**. Those markdown files are source templates. Enable Wikis in Settings → General → Features, then publish each page to the separate Wiki Git repository.
4. GitHub About description and topic tags are repository settings; they are not changed by committing Markdown.
5. Never represent design examples as implemented commands or downloadable SDKs.
6. Assess patent, codec, service access and app store conditions before redistribution.

Suggested About description: `A modular, cross-platform audio conversion engine for MP3, WAV, FLAC, AAC, Opus and more. Built for mobile, desktop, CLI and backend integration.`

Suggested topics: `audio-converter`, `audio-processing`, `ffmpeg`, `rust`, `cross-platform`, `mp3`, `wav`, `flac`, `aac`, `opus`, `cli`, `android`, `ios`, `windows`, `macos`, `linux`, `open-source`.
