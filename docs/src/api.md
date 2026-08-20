# API Reference

Full API docs are generated via `cargo doc`.

```bash
cargo doc --no-deps --open
```

Hosted on docs.rs on `cargo publish`, and on GitHub Pages at `/doc/` (see workflow). Key modules:

- `api::browse::{artist, album, playlist, home_page, home_continue, explore, list_page, browse_with_params}`
- `api::search::{search, search_with_params}`
- `api::watch::{get_song, get_queue, extend_queue, get_lyrics}`
- `api::comments::{comments_page, comment_continuation, reply_page}`
- `api::library::{landing, playlists, songs, albums, artists, subscriptions, history, account_info, rate_song, rate_playlist, add_to_playlist, add_many_to_playlist, remove_from_playlist, create_playlist}`
- `model::{browse, search, watch, comments, library}`
