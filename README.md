<p align="center">
  <h1 align="center">mediyo-core</h1>
  <p align="center">Lightweight YouTube Music core for Rust — search, browse, queue, library.</p>
  <p align="center">
    <a href="https://teamshryne.github.io/mediyo-core/guide/"><img src="https://img.shields.io/badge/docs-guide-blue?style=flat-square" alt="Guide"></a>
    <a href="https://teamshryne.github.io/mediyo-core/doc/mediyo_core/"><img src="https://img.shields.io/badge/docs-api-blue?style=flat-square" alt="API"></a>
    <a href="https://github.com/TeamShryne/mediyo-core/actions"><img src="https://img.shields.io/github/actions/workflow/status/TeamShryne/mediyo-core/docs.yml?style=flat-square" alt="CI"></a>
    <a href="./LICENSE"><img src="https://img.shields.io/badge/license-MIT-lightgrey?style=flat-square" alt="License"></a>
  </p>
</p>

Metadata-only. No streaming solver, no vendored JS. Sync `ureq` + `WEB_REMIX` (`67`).

> Player URLs are out of scope by design.

### Install

```toml
[dependencies]
mediyo-core = { git = "https://github.com/TeamShryne/mediyo-core" }
```

MSRV 1.97.0.

### Quickstart

```rust
use mediyo_core::{Session, api};

let mut session = Session::new();
session.fetch_visitor_data()?;

let results = api::search::search(&session, "sunflower")?;
let artist = api::browse::artist(&session, "UCi8Spc1Fryc45tHLoxVNovg")?;
let song = api::watch::get_song(&session, "58dyibIUscg", None)?;
```

Authenticated:

```rust
let session = Session::new()
    .with_context(mediyo_core::context::Context::new()
        .with_client(mediyo_core::context::Client::new()
            .with_visitor_data(visitor).with_page_id(page_id)))
    .with_cookies(cookies, sapisid);

let pls = api::library::playlists(&session)?;
api::library::add_to_playlist(&session, "VLPL...", "0zmIgxfZz0M")?;
```

### Features

Search, Home/Explore, Artist/Album/Playlist, Watch queue + infinite radio, Lyrics, Comments (Top/Newest + replies), Library (playlists, liked songs, history, account), Mutations (like, save, add to playlist).

Every item carries `browse_id` or `video_id` — `browse(browseId)` to go deeper, `get_queue(videoId)` to play.

### Docs

- **Guide** — https://teamshryne.github.io/mediyo-core/guide/
- **API** — https://teamshryne.github.io/mediyo-core/doc/mediyo_core/

```bash
cargo run --example browse_live
cargo run --example library_live  # needs YTM_COOKIES
cargo test
```

### License

MIT
