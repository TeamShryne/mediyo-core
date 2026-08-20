# Installation

MSRV 1.97.0.

```toml
[dependencies]
mediyo-core = { path = "crates/mediyo-core" }
# or from git
mediyo-core = { git = "https://github.com/TeamShryne/mediyo-core" }
```

No `rquickjs`/`regex`, no vendored JS. `cargo build` and `cargo test` work offline via `research/*.json` fixtures.

```bash
cargo test
cargo clippy --all-targets
cargo fmt --check
```

`research/*.json` are captured fixtures — not needed at runtime.
