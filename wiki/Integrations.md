# Integration Guide

[← Home](Home.md)

TONG Audio Engine contains no GUI. Integration surfaces are planned as follows:

| System | Planned method |
|---|---|
| Android | Kotlin/JNI native library |
| iOS | C ABI / Swift wrapper |
| Windows | DLL or headless EXE |
| macOS | Framework or CLI |
| Linux | Shared library or CLI |
| Backend | CLI worker / optional authenticated REST API |

The examples below are **design proposals** rather than released code.

```text
capabilities() -> codec + output support
probe(input) -> stream metadata
submit(job) -> job_id
status(job_id) -> phase + progress + error
cancel(job_id) -> stop and retain original input
result(job_id) -> verified output
```

[Full integration contract](https://github.com/parmtech/tong-audio-converter/blob/main/docs/INTEGRATION_SPEC.md).
