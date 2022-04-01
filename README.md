[![docs](https://docs.rs/bb8-lapin/badge.svg)](https://docs.rs/bb8-lapin)
[![crates.io-version](https://img.shields.io/crates/v/bb8-lapin)](https://crates.io/crates/bb8-lapin)
[![tests](https://github.com/adrianbenavides/bb8-lapin/workflows/Tests/badge.svg)](https://github.com/adrianbenavides/bb8-lapin/actions)
[![audit](https://github.com/adrianbenavides/bb8-lapin/workflows/Audit/badge.svg)](https://github.com/adrianbenavides/bb8-lapin/actions)
[![crates.io-license](https://img.shields.io/crates/l/bb8-lapin)](LICENSE)

[Lapin](https://github.com/CleverCloud/lapin) support for the [bb8](https://github.com/khuey/bb8) connection pool.

## Usage
See the documentation of bb8 for the details on how to use the connection pool.

```rust
use bb8_lapin::prelude::*;

async fn example() {
    let manager = LapinConnectionManager::new("amqp://guest:guest@127.0.0.1:5672//", ConnectionProperties::default());
    let pool = bb8::Pool::builder()
        .max_size(15)
        .build(manager)
        .await
        .unwrap();
    for _ in 0..20 {
        let pool = pool.clone();
        tokio::spawn(async move {
            let conn = pool.get().await.unwrap();
            // use the connection
            // it will be returned to the pool when it falls out of scope.
        });
    }
}
```

See files in the examples folder for more examples.

## Build-time Requirements
The crate is tested on `ubuntu-latest` against the nightly and stable rust versions.
It is possible that it works with older versions as well but this is not tested.
Please see the details of the bb8 and lapin crates about their requirements.