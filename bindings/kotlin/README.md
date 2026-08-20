# mediyo Kotlin bindings

UniFFI-generated + hand-written JNA wrapper for `mediyo-ffi`.

## Build Rust

```bash
cargo build -p mediyo-ffi --release
# Android (requires cargo-ndk)
cargo ndk -t arm64-v8a -t armeabi-v7a build -p mediyo-ffi --release
# Produces libmediyo_ffi.so
```

Kotlin expects `libmediyo_ffi.so` / `libmediyo_ffi.dylib` / `mediyo_ffi.dll` on `java.library.path`.

## Generate (UniFFI)

```bash
cargo build -p mediyo-ffi --release
# UniFFI 0.28 — generate Kotlin
cargo run --bin uniffi-bindgen -- generate --library target/release/libmediyo_ffi.so --language kotlin --out-dir bindings/kotlin
```

If `uniffi-bindgen` not installed, the hand-written `Mediyo.kt` in this folder is a minimal JNA wrapper — copy it into your Android project:
`app/src/main/java/com/teamshryne/mediyo/Mediyo.kt`

## Usage (Kotlin)

```kotlin
val session = MediyoSession.withCookies("SAPISID=...; SID=...", "cHlUgIcv...")
session.fetchVisitorData()
val res = session.search("sunflower")
val artist = session.browseArtist("UCi8Spc1Fryc45tHLoxVNovg")
val acct = session.accountInfo()
session.rateSong("0zmIgxfZz0M", FfiLikeStatus.Like)
session.addToPlaylist("VLPL...", "0zmIgxfZz0M")
val pid = session.createPlaylist("My mix", "desc")
session.close()
```

See `Mediyo.kt` for all `Ffi*` records. For Flutter, use the same Rust core via `flutter_rust_bridge` with `mediyo-core` directly.
