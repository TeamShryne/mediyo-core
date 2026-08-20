# Mutations

## Like / Save

```rust
use api::library::{LikeStatus, rate_song, rate_playlist};

rate_song(&session, "0zmIgxfZz0M", LikeStatus::Like)?;
rate_song(&session, "0zmIgxfZz0M", LikeStatus::Indifferent)?;

rate_playlist(&session, "PL...", LikeStatus::Like)?; // also OLAK... (album), MPSP... (podcast)
```

Endpoints: `like/like`, `like/dislike`, `like/removelike` with `{"target":{"videoId":...}}` or `{"target":{"playlistId":...}}`.

## Playlist Adds

```rust
// single
api::library::add_to_playlist(&session, "VLPL...", "0zmIgxfZz0M")?;
// many
api::library::add_many_to_playlist(&session, "VLPL...", &["vid1","vid2","vid3"])?;
// remove (needs setVideoId from get_playlist)
api::library::remove_from_playlist(&session, "VLPL...", "setVideoId...", "vid...")?;

let pid = api::library::create_playlist(&session, "mediyo-test", "desc", "PRIVATE")?;
session.request("playlist/delete", json!({"playlistId": pid}))?;
```

Single is sugar for `add_many` with one `ACTION_ADD_VIDEO`. Batch sends one `browse/edit_playlist` with multiple `{"action":"ACTION_ADD_VIDEO","addedVideoId":vid}` actions and returns `playlistEditResults[].playlistEditVideoAddedResultData.newSetVideoId`. Use `DEDUPE_OPTION_SKIP` if you want duplicates allowed (not yet exposed).
