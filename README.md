Rust wrapper for the [Jelastic API](https://docs.jelastic.com/api).

## Install

Set `jelastic-rs` dependency in `Cargo.toml` (see the last version on crates.io):

```toml
...

[dependencies]
jelastic-rs = "0.1.0"
```

## Use

```rust
use jelastic_rs;

#[tokio::main]
async fn main() {
    let jelastic_api = jelastic_rs::Api {
        host: String::from("<JELASTIC API HOST>"),
        token: String::from("<YOUR JELASTIC API TOKEN>"),
    };

    match jelastic_api.start_env("my-env").await {
        Ok(response) => println!("{:#?}", response),
        Err(error) => println!("Error: {}", error),
    }
}
```
