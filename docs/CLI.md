# Experimental working CLI — v0.1.0-alpha.1

The first TONG Audio Engine implementation is a **headless local-file converter** in Rust. It orchestrates installed FFmpeg and FFprobe; no graphical interface, YouTube downloader, SDK or background service is included.

## Requirements
- Rust/Cargo for compiling
- FFmpeg and FFprobe installed and available on PATH
- Audio/video files that you own or may lawfully process

## Commands
~~~bash
cargo test
cargo build --release
cargo run -- capabilities
cargo run -- probe --input ./voice.wav
cargo run -- convert --input ./voice.wav --output ./voice.mp3 --bitrate 192k
cargo run -- convert --input ./voice.wav --output ./voice.flac --sample-rate 48000
cargo run -- extract --input ./owned-video.mp4 --output ./audio.m4a
~~~

Formats are limited to installed FFmpeg encoders. Currently the CLI maps: .mp3 to libmp3lame, .wav to PCM, .flac to FLAC, .m4a/.aac to AAC, .opus to libopus, .ogg to libvorbis, .aiff to PCM and optionally .wma/.ac3.

This is an **early development build**. It checks that input is a local file, passes FFmpeg arguments without shell interpolation, converts into a temporary file, validates resulting audio using ffprobe, and refuses existing outputs. Network source adapters, mobile native SDKs, durable queues, structured job API, secure multi-user worker and strict resource quotas are planned—not currently implemented.

## TONG RADIO MP3-first shortcut

The `mp3` CLI command enforces an `.mp3` output and defaults to **64 kbps**, matching TONG RADIO's bandwidth-conscious Studio pipeline. Explicit values, e.g. `--bitrate 128k`, override the default. The input must already be available locally; URL extraction is a separate adapter.

```bash
cargo run -- mp3 --input ./audio.wav --output ./radio.mp3
cargo run -- mp3 --input ./voice.flac --output ./voice.mp3 --bitrate 128k
```
