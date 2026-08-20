# Introduction

**mediyo-core** is a lightweight, metadata-only YouTube Music core library in Rust. It powers search, browse (home, explore, artist, album, playlist), watch queues, lyrics, comments, and authenticated library/mutations — without vendored JS or streaming solvers.

Built for embedding via FFI into a multi-platform client (mobile/desktop). Sync `ureq` 3.4, `WEB_REMIX` client (`67` / `1.20260818.08.00`), SAPISIDHASH auth.

> **Scope:** metadata only. Player URLs / poToken are out of scope by design.

## Why metadata-only?

Streaming requires per-device JS challenge solving (`rquickjs` + vendored solver) and is brittle. mediyo-core focuses on the stable innertube surface: everything you need to build a full YTM frontend except audio bytes.

## Feature map

- **Search**: songs, albums, artists, playlists, videos, episodes — `filters` + `params` reuse
- **Browse**: `home`/`explore`, `artist` (immersive header + play/radio/share), `album`/`playlist` (header + tracks + carousels), pagination via `next_page`, `home_continue`, `list_page`
- **Watch**: `get_song`/`get_queue` + infinite radio `extend_queue`
- **Lyrics**: `get_lyrics` from `Song.lyrics_browse_id`
- **Comments**: 20/page, `Top`/`Newest` sort, replies, continuations
- **Library (auth)**: `landing`, `playlists`, `songs`, `albums`, `artists`, `subscriptions`, `history`, `account_info`
- **Mutations (auth)**: `rate_song`/`rate_playlist`, `add_to_playlist`/`add_many_to_playlist`, `remove_from_playlist`, `create_playlist`
