# thetvdb

[![CI](https://img.shields.io/github/workflow/status/roignpar/thetvdb/CI)][ci]
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.40%2B-informational)][rustc]

__[TheTVDB]__ [API V3] Rust async client.

__Minimum Rust version: `1.40.0`__

In order to use this client you will need an API key. To create a new API key
[log in] and go to the [API Keys page].

__NOTE__: Because it depends on [reqwest], thetvdb currently only works with
[tokio].

### Install

Add with [cargo edit]:
```
cargo add thetvdb
```

_or_ add to `Cargo.toml`:
```toml
[dependencies]
thetvdb = "0.1.0"
```

### Use
Search series by title:
```rust
use thetvdb::{Client, params::SearchBy};

let client = Client::new("YOUR_API_KEY").await?;
let search_results = client.search(SearchBy::Name("Planet Earth")).await?;
```
For more examples check the [documentation].

### Supported requests:

#### Authentication
- [x] `POST /login` (used internally; cannot be manually requested)

#### Episodes
- [x] `GET /episodes/{id}`

#### Languages
- [x] `GET /languages`
- [x] `GET /languages/{id}`

#### Movies
- [x] `GET /movies/{id}`
- [ ] `GET /movieupdates`

#### Search
- [x] `GET /search/series`

#### Series
- [x] `GET /series/{id}`
- [x] `HEAD /series/{id}`
- [x] `GET /series/{id}/actors`
- [x] `GET /series/{id}/episodes`
- [x] `GET /series/{id}/episodes/query`
- [x] `GET /series/{id}/episodes/summary`
- [x] `GET /series/{id}/filter`
- [x] `GET /series/{id}/images`
- [x] `GET /series/{id}/images/query`
- [x] `GET /series/{id}/images/query/params`

#### Updates
- [x] `GET /updated/query`

### Integration tests
```
export THETVDB_APIKEY=<API_KEY>
cargo t --test client
```

### License
Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT](LICENSE-MIT) at your option.

[ci]: https://github.com/roignpar/thetvdb/actions?query=workflow%3ACI
[rustc]: https://blog.rust-lang.org/2019/12/19/Rust-1.40.0.html
[log in]: https://thetvdb.com/auth/login
[api keys page]: https://thetvdb.com/dashboard/account/apikeys
[reqwest]: https://github.com/seanmonstar/reqwest
[tokio]: https://github.com/tokio-rs/tokio
[cargo edit]: https://github.com/killercup/cargo-edit
[thetvdb]: https://thetvdb.com/
[api v3]: https://api.thetvdb.com/swagger
[documentation]: https://docs.rs/thetvdb/latest
