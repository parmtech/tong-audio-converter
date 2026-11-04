# Contributing to TONG Audio Engine

Thank you for contributing to a modular, open-source audio engine.

## Before submitting

1. Review [architecture](docs/ARCHITECTURE.md), [roadmap](docs/ROADMAP.md) and [license compliance](docs/LICENSE_COMPLIANCE.md).
2. Open a focused issue for nontrivial changes describing target platform, codecs, test cases and risk.
3. Keep GUI, DRM bypass and unauthorized-source features out of the engine.
4. Do not paste proprietary code or submit third-party material without compatible rights and attribution.
5. Add tests for failures (cancel, disk full, no codec, corrupt input, repeat job, malformed metadata) in addition to success.
6. Avoid logging credentials, cookies, tokens or private media URLs.
7. Provide benchmarks with hardware/software details when claiming speed improvements.

## Architecture principles

Small, versioned public interfaces. No hidden cloud dependency. Opt-in source integrations. Accurate build capability manifests. Local-media conversion always independent of third-party websites.

## Issue reporting

State environment, exact build hash, codec manifest, steps to reproduce, minimal redistributable sample and redacted logs. Please use the issue templates.