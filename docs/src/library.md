# Library

All calls require an authenticated `Session` (see Authentication).

```rust
let landing = api::library::landing(&session)?; // 3 tiles: Liked Music (VLLM), custom playlists, Episodes (VLSE)
let pls = api::library::playlists(&session)?; // FEmusic_liked_playlists (grid)
let songs = api::library::songs(&session)?; // FEmusic_liked_videos (musicShelf, 4 liked)
let albums = api::library::albums(&session)?;
let artists = api::library::artists(&session)?; // FEmusic_library_corpus_track_artists
let subs = api::library::subscriptions(&session)?; // FEmusic_library_corpus_artists
let hist = api::library::history(&session)?; // FEmusic_history
let acct = api::library::account_info(&session)?; // { name, handle, photo_url }
```

`LibraryPage { items: Vec<SearchResult>, continuation: Option<String> }` — `items` carry `browse_id` (playlists `VLLM`/`VLPL...`) or `video_id` (songs). Paginate via `LibraryPage.continuation` with `browse_continuation` / `gridContinuation` / `musicShelfContinuation` internally.

Landing example: `VLLM` is the auto Liked Music playlist, `VLSE` is Episodes for Later.
