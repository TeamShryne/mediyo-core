# Authentication

Anonymous:

```rust
let mut session = Session::new();
session.fetch_visitor_data()?;
let home = api::browse::home_page(&session)?;
```

Authenticated (cookies):

```rust
let cookie = std::env::var("YTM_COOKIES")?; // full Cookie header
let sapisid = cookie.split(';').find_map(|p| p.trim().strip_prefix("SAPISID=")).map(|s| s.to_string());

// Visitor-Id + PageId from an authenticated browse (required for library)
let visitor = std::fs::read_to_string("/tmp/visitor.txt")?.trim().to_string();
let page_id = std::fs::read_to_string("/tmp/pageid.txt")?.trim().to_string();

let client = mediyo_core::context::Client::new()
    .with_visitor_data(visitor)
    .with_page_id(page_id);
let session = Session::new()
    .with_context(mediyo_core::context::Context::new().with_client(client))
    .with_cookies(cookie, sapisid);
```

`Session` sends `Cookie`, `SAPISIDHASH` (`SHA1(ts + " " + SAPISID + " " + "https://music.youtube.com")`), `X-Goog-Visitor-Id`, `X-Goog-PageId`, `X-Youtube-Bootstrap-Logged-In`, `X-Goog-AuthUser: 2` plus `WEB_REMIX` headers. All `api::*` calls use the same `Session` — pass an authenticated one for library/mutations.

Never commit `cookies.txt` / `visitor.txt` — they contain `SID`/`SAPISID`/`LOGIN_INFO`.
