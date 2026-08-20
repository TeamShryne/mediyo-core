# Search & Browse

## Search

```rust
let resp = api::search::search(&session, "sunflower")?;
for r in &resp.results { println!("{} — {:?}", r.title, r.category); }
// filters carry params for scoped search
let filtered = api::search::search_with_params(&session, "sunflower", resp.filters[0].params.as_deref())?;
```

## Home & Explore

```rust
let mut home = api::browse::home_page(&session)?;
while let Some(tok) = home.continuation.take() { home = api::browse::home_continue(&session, &tok)?; }

let explore = api::browse::explore(&session)?; // FEmusic_explore
for btn in &explore.nav_buttons { println!("{} -> {}", btn.label, btn.browse_id); } // New releases, Charts...
```

## Artist / Album / Playlist

```rust
let artist = api::browse::artist(&session, "UC...")?;
// artist.play_button / radio_button -> WatchEndpoint { video_id?, playlist_id, params }
// artist.carousels[].view_all -> list_page(browseId, params)
let view_all = &artist.carousels.iter().find(|c| c.title=="Singles & EPs").unwrap().view_all;
let page = api::browse::list_page(&session, &view_all.as_ref().unwrap().browse_id, view_all.as_ref().unwrap().params.as_deref())?;

let album = api::browse::album(&session, "MPRE...")?;
let playlist = api::browse::playlist(&session, "VLPL...")?;
let mut total = playlist.tracks.len();
while let Some(tok) = playlist.continuation.take() {
    let next = api::browse::next_page(&session, &tok)?;
}
```

## Watch & Lyrics & Comments

```rust
let song = api::watch::get_song(&session, "58dyibIUscg", Some("RDAMVM58dyibIUscg"))?;
if let Some(bid) = &song.lyrics_browse_id { let lyrics = api::watch::get_lyrics(&session, bid)?; }

let next = api::watch::next(&session, "58dyibIUscg", None)?;
let tok = mediyo_core::model::comments_token(&next).unwrap();
let mut page = api::comments::comments_page(&session, &tok)?;
let newest = page.sort_filters.iter().find(|f| f.title=="Newest").unwrap();
let page = api::comments::comments_page(&session, &newest.continuation_token)?;
```

Every `Carousel.items` is `Vec<SearchResult>` (`browse_id`/`video_id`/`playlist_id` + `view_all` + `continuation`). Use `browse(browseId)` or `get_queue(videoId, playlistId)` to navigate.
