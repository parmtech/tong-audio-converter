use std::{env, fs, path::{Path, PathBuf}, process::{Command, ExitCode}, time::{SystemTime, UNIX_EPOCH}};

const HELP: &str = "TONG Audio Engine (experimental, local media only)\n\
Commands:\n  tong-audio capabilities\n  tong-audio probe --input FILE\n\
  tong-audio mp3 --input FILE --output FILE.mp3 [--bitrate 64k]\n\\\n  tong-audio convert --input FILE --output FILE [--bitrate 192k] [--sample-rate 44100] [--channels 2]\n\
  tong-audio extract --input VIDEO --output AUDIO [same options]\n\
  tong-audio --help | --version\n\
Requires ffmpeg/ffprobe in PATH or TONG_FFMPEG/TONG_FFPROBE. Output is never overwritten.\n";

#[derive(Default)]
struct Opts {
    input: Option<PathBuf>, output: Option<PathBuf>,
    bitrate: Option<String>, rate: Option<u32>, channels: Option<u8>,
}
fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => { eprintln!("TONG_ERROR: {e}"); ExitCode::FAILURE }
    }
}
fn binary(key: &str, fallback: &str) -> String {
    env::var(key).ok().filter(|s| !s.is_empty()).unwrap_or_else(|| fallback.into())
}
fn options(args: &[String]) -> Result<Opts, String> {
    let mut o = Opts::default();
    let mut i = 0;
    while i < args.len() {
        let key = args[i].as_str();
        let val = args.get(i + 1).ok_or_else(|| format!("missing value for {key}"))?;
        match key {
            "--input" => { if o.input.is_some() { return Err("duplicate --input".into()); } o.input = Some(val.into()); }
            "--output" => { if o.output.is_some() { return Err("duplicate --output".into()); } o.output = Some(val.into()); }
            "--bitrate" => {
                let n = val.strip_suffix('k').and_then(|s| s.parse::<u16>().ok());
                if !n.map(|n| (8..=512).contains(&n)).unwrap_or(false) { return Err("bitrate must be 8k..512k".into()); }
                o.bitrate = Some(val.clone());
            }
            "--sample-rate" => {
                let n = val.parse::<u32>().map_err(|_| "bad sample rate")?;
                if !(8000..=192000).contains(&n) { return Err("sample rate outside 8000..192000".into()); }
                o.rate = Some(n);
            }
            "--channels" => {
                let n = val.parse::<u8>().map_err(|_| "bad channel count")?;
                if !(1..=8).contains(&n) { return Err("channels outside 1..8".into()); }
                o.channels = Some(n);
            }
            _ => return Err(format!("unknown option {key}")),
        }
        i += 2;
    }
    Ok(o)
}
fn source(o: &Opts) -> Result<&Path, String> {
    let p = o.input.as_deref().ok_or("missing --input")?;
    if !fs::metadata(p).map_err(|e| e.to_string())?.is_file() { return Err("input must be a regular local file".into()); }
    Ok(p)
}
fn codec(ext: &str) -> Option<&'static str> {
    match ext {
        "mp3" => Some("libmp3lame"), "wav" => Some("pcm_s16le"),
        "flac" => Some("flac"), "m4a" | "aac" => Some("aac"),
        "opus" => Some("libopus"), "ogg" => Some("libvorbis"),
        "aif" | "aiff" => Some("pcm_s16be"), "ac3" => Some("ac3"),
        "wma" => Some("wmav2"), _ => None,
    }
}
fn convert(o: Opts) -> Result<(), String> {
    let input = source(&o)?.to_owned();
    let out = o.output.as_deref().ok_or("missing --output")?;
    let ext = out.extension().and_then(|e| e.to_str()).ok_or("output requires extension")?.to_ascii_lowercase();
    let encoder = codec(&ext).ok_or_else(|| format!("unsupported output extension: {ext}"))?;
    if out.exists() { return Err("OUTPUT_EXISTS: target is not overwritten".into()); }
    let dir = out.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
    if !dir.is_dir() { return Err("output directory missing".into()); }
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| e.to_string())?.as_nanos();
    let tmp = dir.join(format!(".tong-{}-{seed}.{ext}", std::process::id()));
    let mut ff = Command::new(binary("TONG_FFMPEG", "ffmpeg"));
    ff.args(["-nostdin", "-hide_banner", "-loglevel", "error", "-n", "-i"])
        .arg(input).args(["-map", "0:a:0", "-vn", "-c:a", encoder]);
    if let Some(ref bit) = o.bitrate { ff.args(["-b:a", bit]); }
    if let Some(rate) = o.rate { ff.args(["-ar", &rate.to_string()]); }
    if let Some(channels) = o.channels { ff.args(["-ac", &channels.to_string()]); }
    let result = ff.arg(&tmp).status().map_err(|e| format!("ffmpeg unavailable: {e}"))?;
    if !result.success() { let _ = fs::remove_file(&tmp); return Err("CONVERSION_FAILED: check FFmpeg diagnostics".into()); }
    let verification = Command::new(binary("TONG_FFPROBE", "ffprobe"))
        .args(["-v", "error", "-select_streams", "a:0", "-show_entries", "stream=codec_name",
            "-of", "default=noprint_wrappers=1:nokey=1"])
        .arg(&tmp).output().map_err(|e| format!("ffprobe unavailable: {e}"));
    let valid = verification.as_ref().map(|x| x.status.success() && !x.stdout.is_empty()).unwrap_or(false);
    if !valid || out.exists() { let _ = fs::remove_file(&tmp); return Err("OUTPUT_INVALID or OUTPUT_EXISTS".into()); }
    fs::rename(&tmp, out).map_err(|e| format!("failed to publish output: {e}"))?;
    println!("OUTPUT_OK: {}", out.display());
    Ok(())
}
fn run(args: Vec<String>) -> Result<(), String> {
    match args.first().map(|s| s.as_str()) {
        None | Some("help" | "--help" | "-h") => { print!("{HELP}"); Ok(()) }
        Some("version" | "--version") => { println!("tong-audio {}", env!("CARGO_PKG_VERSION")); Ok(()) }
        Some("capabilities") => {
            let status = Command::new(binary("TONG_FFMPEG", "ffmpeg"))
                .args(["-hide_banner", "-encoders"]).status().map_err(|e| e.to_string())?;
            if status.success() { Ok(()) } else { Err("ffmpeg query failed".into()) }
        }
        Some("probe") => {
            let o = options(&args[1..])?;
            if o.output.is_some() { return Err("probe does not take --output".into()); }
            let input = source(&o)?;
            let output = Command::new(binary("TONG_FFPROBE", "ffprobe"))
                .args(["-v", "error", "-show_format", "-show_streams", "-of", "json"])
                .arg(input).output().map_err(|e| format!("ffprobe unavailable: {e}"))?;
            if !output.status.success() { return Err(String::from_utf8_lossy(&output.stderr).to_string()); }
            print!("{}", String::from_utf8_lossy(&output.stdout));
            Ok(())
        }
        Some("mp3") => {
            let mut o = options(&args[1..])?;
            let output = o.output.as_deref().ok_or("missing --output")?;
            if output.extension().and_then(|x| x.to_str()).map(|x| x.eq_ignore_ascii_case("mp3")) != Some(true) {
                return Err("mp3 requires an .mp3 output filename".into());
            }
            if o.bitrate.is_none() { o.bitrate = Some("64k".into()); }
            convert(o)
        }
        Some("convert" | "extract") => convert(options(&args[1..])?),
        Some(other) => Err(format!("unknown command {other}; see --help")),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn formats() { assert_eq!(codec("mp3"), Some("libmp3lame")); assert_eq!(codec("flac"), Some("flac")); assert_eq!(codec("exe"), None); }
    #[test] fn errors() {
        assert!(options(&["--input".into()]).is_err());
        assert!(options(&["--bitrate".into(), "0k".into()]).is_err());
        assert!(options(&["--sample-rate".into(), "bad".into()]).is_err());
    }
}
