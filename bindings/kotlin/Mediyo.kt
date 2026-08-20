package com.teamshryne.mediyo

import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer

// UniFFI-generated bindings for mediyo-ffi
// Build Rust: cargo build -p mediyo-ffi --release
// Then load libmediyo_ffi.so / .dylib / .dll

data class FfiSearchResult(
    val title: String,
    val videoId: String?,
    val browseId: String?,
    val playlistId: String?,
    val category: String
)

data class FfiSearchResponse(val results: List<FfiSearchResult>)
data class FfiArtistPage(val name: String, val subscriberCount: String?, val monthlyAudience: String?, val description: String?)
data class FfiAccountInfo(val name: String, val handle: String?, val photoUrl: String?)
data class FfiLibraryPage(val titles: List<String>)

enum class FfiLikeStatus { Like, Dislike, Indifferent }

class MediyoException(message: String) : Exception(message)

class MediyoSession private constructor(private val ptr: Pointer) : AutoCloseable {
    companion object {
        init { System.loadLibrary("mediyo_ffi") }
        @JvmStatic fun new(): MediyoSession = MediyoSession(NativeLibrary.INSTANCE.mediyo_session_new())
        @JvmStatic fun withCookies(cookie: String, sapisid: String?): MediyoSession =
            MediyoSession(NativeLibrary.INSTANCE.mediyo_session_with_cookies(cookie, sapisid))
    }
    fun fetchVisitorData(): String = NativeLibrary.INSTANCE.mediyo_session_fetch_visitor_data(ptr).getOrThrow()
    fun search(query: String): FfiSearchResponse = NativeLibrary.INSTANCE.mediyo_search(ptr, query).getOrThrow()
    fun browseArtist(browseId: String): FfiArtistPage = NativeLibrary.INSTANCE.mediyo_browse_artist(ptr, browseId).getOrThrow()
    fun accountInfo(): FfiAccountInfo = NativeLibrary.INSTANCE.mediyo_account_info(ptr).getOrThrow()
    fun libraryPlaylists(): FfiLibraryPage = NativeLibrary.INSTANCE.mediyo_library_playlists(ptr).getOrThrow()
    fun librarySongs(): FfiLibraryPage = NativeLibrary.INSTANCE.mediyo_library_songs(ptr).getOrThrow()
    fun rateSong(videoId: String, status: FfiLikeStatus) = NativeLibrary.INSTANCE.mediyo_rate_song(ptr, videoId, status).getOrThrow()
    fun addToPlaylist(playlistId: String, videoId: String) = NativeLibrary.INSTANCE.mediyo_add_to_playlist(ptr, playlistId, videoId).getOrThrow()
    fun createPlaylist(title: String, description: String): String = NativeLibrary.INSTANCE.mediyo_create_playlist(ptr, title, description).getOrThrow()
    override fun close() { NativeLibrary.INSTANCE.mediyo_session_free(ptr) }
}

// Example:
// val session = MediyoSession.withCookies("SAPISID=...; SID=...", "cHlUgIcv...")
// session.fetchVisitorData()
// val results = session.search("sunflower")
// val artist = session.browseArtist("UCi8Spc1Fryc45tHLoxVNovg")

private interface NativeLibrary : Library {
    fun mediyo_session_new(): Pointer
    fun mediyo_session_with_cookies(cookie: String, sapisid: String?): Pointer
    fun mediyo_session_fetch_visitor_data(ptr: Pointer): Result<String>
    fun mediyo_search(ptr: Pointer, query: String): Result<FfiSearchResponse>
    fun mediyo_browse_artist(ptr: Pointer, browseId: String): Result<FfiArtistPage>
    fun mediyo_account_info(ptr: Pointer): Result<FfiAccountInfo>
    fun mediyo_library_playlists(ptr: Pointer): Result<FfiLibraryPage>
    fun mediyo_library_songs(ptr: Pointer): Result<FfiLibraryPage>
    fun mediyo_rate_song(ptr: Pointer, videoId: String, status: FfiLikeStatus): Result<Unit>
    fun mediyo_add_to_playlist(ptr: Pointer, playlistId: String, videoId: String): Result<Unit>
    fun mediyo_create_playlist(ptr: Pointer, title: String, description: String): Result<String>
    fun mediyo_session_free(ptr: Pointer)
    companion object {
        val INSTANCE: NativeLibrary = Native.load("mediyo_ffi", NativeLibrary::class.java)
    }
    class Result<T> {
        fun getOrThrow(): T = throw NotImplementedError("UniFFI Result")
    }
}
