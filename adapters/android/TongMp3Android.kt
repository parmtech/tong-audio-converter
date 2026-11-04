package com.parmtech.tongaudio

import android.content.Context
import android.media.MediaMetadataRetriever
import com.yausername.ffmpeg.FFmpeg
import com.yausername.youtubedl_android.YoutubeDL
import com.yausername.youtubedl_android.YoutubeDLRequest
import java.io.File
import org.json.JSONObject

/**
 * Headless Android MP3 source adapter for TONG Audio Engine.
 * Extractor transport is separate from the portable Rust/FFmpeg core.
 * This source is Apache-2.0; youtubedl-android itself is GPL-3.0.
 */
object TongMp3Android {
    data class Output(val file: File, val title: String, val durationMs: Long, val extractorVersion: String?)
    class ConversionFailure(val code: String, detail: String, cause: Throwable? = null) :
        IllegalStateException(code + ": " + detail, cause)

    fun errorCode(message: String): String {
        val m = message.lowercase()
        return when {
            "sabr" in m || "missing a url" in m || "missing url" in m -> "SOURCE_SABR_UNAVAILABLE"
            "sign in" in m || "login required" in m || "private video" in m || "age-restricted" in m -> "SOURCE_RESTRICTED"
            "http error 403" in m || "forbidden" in m -> "SOURCE_ACCESS_DENIED"
            "requested format is not available" in m || "no video formats found" in m -> "SOURCE_FORMAT_UNAVAILABLE"
            "network is unreachable" in m || "timed out" in m -> "SOURCE_NETWORK"
            else -> "SOURCE_EXTRACTOR_ERROR"
        }
    }

    fun verifyMp3(file: File): Long? {
        if (!file.isFile || file.length() < 1024) return null
        val meta = MediaMetadataRetriever()
        return try {
            meta.setDataSource(file.absolutePath)
            val mime = meta.extractMetadata(MediaMetadataRetriever.METADATA_KEY_MIMETYPE)
                ?.lowercase().orEmpty()
            if (!mime.contains("mpeg") && !mime.contains("mp3")) return null
            meta.extractMetadata(MediaMetadataRetriever.METADATA_KEY_DURATION)
                ?.toLongOrNull()?.takeIf { it > 0L }
        } catch (_: Exception) {
            null
        } finally {
            meta.release()
        }
    }

    private fun refresh(ctx: Context, force: Boolean) {
        val p = ctx.getSharedPreferences("tong_audio_extractor", Context.MODE_PRIVATE)
        val now = System.currentTimeMillis()
        val last = p.getLong("last_update_check", 0L)
        if (!force && now - last in 0L..86_399_999L) return
        try {
            YoutubeDL.getInstance().updateYoutubeDL(ctx, YoutubeDL.UpdateChannel.STABLE)
            p.edit().putLong("last_update_check", now).apply()
        } catch (_: Exception) {
            // A temporary updater failure must not disable a working installed extractor.
        }
    }

    /** Run on a worker thread. Pass host-validated, authorized YouTube links only. */
    fun convertYoutube(ctx: Context, url: String, work: File, target: File,
                       processId: String, bitrateKbps: Int = 64): Output {
        require(bitrateKbps in listOf(64, 96, 128, 160, 192, 256, 320))
        require(url.startsWith("https://www.youtube.com/watch?v="))
        val dl = YoutubeDL.getInstance()
        dl.init(ctx)
        FFmpeg.getInstance().init(ctx)
        refresh(ctx, false)

        fun attempt(client: String?) {
            work.deleteRecursively()
            check(work.mkdirs() || work.isDirectory) { "Cannot prepare temporary directory" }
            val request = YoutubeDLRequest(url)
            request.addOption("--no-playlist")
            request.addOption("--match-filter", "!is_live")
            request.addOption("--format", "bestaudio/best")
            request.addOption("--extract-audio")
            request.addOption("--audio-format", "mp3")
            request.addOption("--audio-quality", bitrateKbps.toString() + "K")
            request.addOption("--write-info-json")
            request.addOption("--retries", 2)
            request.addOption("--fragment-retries", 2)
            request.addOption("--socket-timeout", 20)
            if (client != null) request.addOption("--extractor-args", "youtube:player_client=" + client)
            request.addOption("-o", File(work, "track.%(ext)s").absolutePath)
            dl.execute(request, processId, false, null)
        }

        try {
            try {
                attempt(null)
            } catch (first: Exception) {
                val category = errorCode(first.message.orEmpty())
                if (category !in listOf("SOURCE_SABR_UNAVAILABLE", "SOURCE_FORMAT_UNAVAILABLE")) throw first
                // Some videos expose a permitted HLS stream through web_safari.
                // Neither retry nor yt-dlp update guarantees upstream access.
                refresh(ctx, true)
                attempt("web_safari")
            }
            val audio = File(work, "track.mp3")
            val duration = verifyMp3(audio)
                ?: throw ConversionFailure("INVALID_MP3", "Extractor did not produce playable MP3")
            val title = runCatching {
                JSONObject(File(work, "track.info.json").readText()).optString("title", "Audio")
            }.getOrDefault("Audio").take(150)
            val parent = target.parentFile
                ?: throw ConversionFailure("OUTPUT_ERROR", "Audio Library directory is missing")
            if (!parent.isDirectory && !parent.mkdirs())
                throw ConversionFailure("OUTPUT_ERROR", "Cannot create Audio Library directory")
            val partial = File(parent, target.name + ".partial")
            partial.delete()
            try {
                audio.copyTo(partial, overwrite = false)
                check(verifyMp3(partial) != null) { "Copied MP3 failed verification" }
                if (target.exists() && !target.delete())
                    throw ConversionFailure("OUTPUT_ERROR", "Cannot replace invalid previous audio")
                check(partial.renameTo(target)) { "Cannot move MP3 to Audio Library" }
            } finally {
                partial.delete()
            }
            return Output(target, title, duration, dl.versionName(ctx))
        } catch (e: ConversionFailure) {
            throw e
        } catch (e: Exception) {
            val code = errorCode(e.message.orEmpty())
            val detail = when (code) {
                "SOURCE_SABR_UNAVAILABLE" -> "YouTube did not expose a usable stream URL; MP3 encoder is not at fault"
                "SOURCE_FORMAT_UNAVAILABLE" -> "No compatible audio stream available"
                "SOURCE_RESTRICTED" -> "Media requires access permissions"
                "SOURCE_ACCESS_DENIED" -> "Upstream denied access"
                "SOURCE_NETWORK" -> "Network access failed"
                else -> "Source extraction failed; check upstream extractor and availability"
            }
            throw ConversionFailure(code, detail, e)
        } finally {
            work.deleteRecursively()
        }
    }
}
