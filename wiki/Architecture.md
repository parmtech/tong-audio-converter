# Architecture

[← Home](Home.md)

```mermaid
flowchart TD
    A[Host app / CLI / worker] --> B[Stable API contract]
    B --> C[Bounded job supervisor]
    C --> D[Authorized source adapters]
    D --> E[Probe / validate]
    E --> F[Codec backend]
    F --> G[Audio processing]
    G --> H[Atomic output]
    H --> I[Verify + checksum]
```

The central orchestration layer is planned in Rust, exposing a C ABI and thin platform wrappers. FFmpeg provides practical broad conversion capabilities; third-party URL extractors remain optional and independent.

Read the [full architecture specification](https://github.com/parmtech/tong-audio-converter/blob/main/docs/ARCHITECTURE.md).
