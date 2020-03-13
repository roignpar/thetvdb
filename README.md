# thetvdb

[![crates.io](https://img.shields.io/crates/v/thetvdb.svg)][crate]
[![Released API docs](https://docs.rs/thetvdb/badge.svg)][documentation]
[![License](https://img.shields.io/crates/l/thetvdb/0.1.0-beta.2)](LICENSE-MIT)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.40%2B-informational)][rustc]
[![CI](https://img.shields.io/github/workflow/status/roignpar/thetvdb/CI)][ci]

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
thetvdb = "0.1.0-beta.2"
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
* `POST /login` (used internally; cannot be manually requested)

#### Episodes
* `GET /episodes/{id}`

#### Languages
* `GET /languages`
* `GET /languages/{id}`

#### Movies
* `GET /movies/{id}`
* `GET /movieupdates`

#### Search
* `GET /search/series`

#### Series
* `GET /series/{id}`
* `HEAD /series/{id}`
* `GET /series/{id}/actors`
* `GET /series/{id}/episodes`
* `GET /series/{id}/episodes/query`
* `GET /series/{id}/episodes/summary`
* `GET /series/{id}/filter`
* `GET /series/{id}/images`
* `GET /series/{id}/images/query`
* `GET /series/{id}/images/query/params`

#### Updates
* `GET /updated/query`

### Integration tests
```
export THETVDB_APIKEY=<API_KEY>
cargo t --test client
```

### License
Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT](LICENSE-MIT) at your option.

[crate]: https://crates.io/crates/thetvdb
[ci]: https://github.com/roignpar/thetvdb/actions?query=workflow%3ACI
[rustc]: https://blog.rust-lang.org/2019/12/19/Rust-1.40.0.html
[log in]: https://thetvdb.com/auth/login
[api keys page]: https://thetvdb.com/dashboard/account/apikeys
[reqwest]: https://github.com/seanmonstar/reqwest
[tokio]: https://github.com/tokio-rs/tokio
[cargo edit]: https://github.com/killercup/cargo-edit
[thetvdb]: https://thetvdb.com/
[api v3]: https://api.thetvdb.com/swagger
[documentation]: https://docs.rs/thetvdb
