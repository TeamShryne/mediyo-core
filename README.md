# mediyo-core

Lightweight, metadata-only YouTube Music core library in Rust — search, browse, watch queues, lyrics, comments, explore, and authenticated library/mutations. No vendored JS, no streaming solver.

Built for embedding via FFI into a multi-platform client (mobile/desktop). Sync `ureq` 3.4, `WEB_REMIX` client (`67` / `1.20260818.08.00`), SAPISIDHASH auth.

> **Scope:** metadata only. Player URLs / poToken are out of scope by design.

## Features

| Area | What | Pagination / Continuation |
|------|------|---------------------------|
| **Search** | songs, albums, artists, playlists, videos, episodes — `filters` + `params` reuse | — |
| **Browse** | `home` (carousels + `continuation`), `explore` (`FEmusic_explore` → nav buttons + 5 carousels), `artist` (immersive header + top songs + carousels + view-all), `album`/`playlist` (header + tracks + carousels) | `next_page(token)`, `home_continue`, `browse_continuation`, `list_page(browseId, params)` |
| **Watch** | `get_song` / `get_queue` (+ `playlistId`), infinite radio | `extend_queue(token)` / `next_continuation` |
| **Lyrics** | `get_lyrics(lyrics_browse_id)` from `Song.lyrics_browse_id` | — |
| **Comments** | `comments_page` (20/page), header count, `Top`/`Newest` sort | `comment_continuation`, `reply_page(replies_continuation)` |
| **Library (auth)** | `landing`, `playlists` (`FEmusic_liked_playlists`), `songs` (`FEmusic_liked_videos`), `albums`, `artists`, `subscriptions`, `history`, `account_info` | `LibraryPage.continuation` |
| **Mutations (auth)** | `rate_song` / `rate_playlist` (`Like`/`Dislike`/`Indifferent`), `add_to_playlist` / `add_many_to_playlist`, `remove_from_playlist`, `create_playlist`, `playlist/delete` | — |
| **Artist extras** | `WatchEndpoint { video_id?, playlist_id, params }` for play/radio buttons, `share_entity` | — |

All `Carousel` items are `SearchResult` (`browse_id`/`video_id`/`playlist_id` + `view_all` + `continuation`). Every item is navigable: `browse(browseId)` or `get_queue(videoId, playlistId)`.

## Installation

```toml
[dependencies]
mediyo-core = { path = "crates/mediyo-core" }
```

MSRV 1.97.0. No `rquickjs`/`regex`, no vendored JS.

## Quickstart

```rust
use mediyo_core::{Session, api};

let mut session = Session::new();
session.fetch_visitor_data()?;

// Search
let resp = api::search::search(&session, "sunflower")?;
for r in &resp.results { println!("{} — {:?}", r.title, r.category); }

// Home + pagination
let mut home = api::browse::home_page(&session)?;
while let Some(tok) = home.continuation.take() {
    home = api::browse::home_continue(&session, &tok)?;
}

// Artist + view-all discography
let artist = api::browse::artist(&session, "UCi8Spc1Fryc45tHLoxVNovg")?;
if let Some(view_all) = &artist.carousels.iter().find(|c| c.title=="Singles & EPs").unwrap().view_all {
    let page = api::browse::list_page(&session, &view_all.browse_id, view_all.params.as_deref())?;
}

// Queue + lyrics
let song = api::watch::get_song(&session, "58dyibIUscg", Some("RDAMVM58dyibIUscg"))?;
if let Some(bid) = &song.lyrics_browse_id {
    let lyrics = api::watch::get_lyrics(&session, bid)?;
}

// Comments (Top/Newest + replies)
let next = api::watch::next(&session, "58dyibIUscg", None)?;
let tok = mediyo_core::model::comments_token(&next).unwrap();
let mut page = api::comments::comments_page(&session, &tok)?;
let newest = page.sort_filters.iter().find(|f| f.title=="Newest").unwrap();
let page = api::comments::comments_page(&session, &newest.continuation_token)?;

// Explore
let explore = api::browse::explore(&session)?;
for btn in &explore.nav_buttons { println!("{} -> {}", btn.label, btn.browse_id); }
```

## Authenticated

```rust
let cookie = std::env::var("YTM_COOKIES")?; // full Cookie header
let sapisid = cookie.split(';').find_map(|p| p.trim().strip_prefix("SAPISID=")).map(|s| s.to_string());
// Visitor-Id + PageId from an authenticated browse (curl) — required for library
let visitor = std::fs::read_to_string("/tmp/visitor.txt")?.trim().to_string();
let page_id = std::fs::read_to_string("/tmp/pageid.txt")?.trim().to_string();

let client = mediyo_core::context::Client::new().with_visitor_data(visitor).with_page_id(page_id);
let session = Session::new().with_context(mediyo_core::context::Context::new().with_client(client))
    .with_cookies(cookie, sapisid);

// Library
let pls = api::library::playlists(&session)?;
let songs = api::library::songs(&session)?;
let acct = api::library::account_info(&session)?; // { name, handle, photo_url }

// Mutations
api::library::rate_song(&session, "0zmIgxfZz0M", api::library::LikeStatus::Like)?;
api::library::rate_playlist(&session, "PL...", api::library::LikeStatus::Like)?; // also OLAK... / MPSP...
api::library::add_many_to_playlist(&session, "VLPL...", &["vid1","vid2"])?;
```

Library mutations use `like/*` and `browse/edit_playlist` (`ACTION_ADD_VIDEO`, `ACTION_REMOVE_VIDEO` with `setVideoId`).

## Examples

```bash
cargo run --example search_live
cargo run --example browse_live
cargo run --example watch_live
cargo run --example comments_live
# auth
YTM_COOKIES="..." YTM_VISITOR_ID="..." YTM_PAGE_ID="..." cargo run --example library_live
cargo run --example library_mutation_live
```

## Testing

`research/*.json` are captured fixtures (no credentials). Tests run offline:

```bash
cargo test
cargo clippy --all-targets
cargo fmt --check
```

21 tests: search/browse/watch/comments/library/explore.

## Project layout

```
crates/mediyo-core/src/
  lib.rs, session.rs, context.rs, auth.rs, error.rs
  api/{browse,search,watch,comments,library}.rs
  model/{browse,search,watch,comments,library}.rs
  parser/{runs,thumbnails,renderer}.rs
research/*.json  # fixtures
```

See `crates/mediyo-core/README.md` for crate-level docs.

## License

MIT
