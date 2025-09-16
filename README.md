# nhl_stats

This crate provides convenient access to the NHL API for applications written within Rust. To my knowledge the previous api 
```bash
"https://statsapi.web.nhl.com/" 
```
has become obsolete and now 
```bash
"https://api-web.nhle.com/"
```
is in use. The currently available crates for NHL statistics all seem to use the former. 
As part of learning Rust I though this would be a good small scale project to test myself on what is within the rust-book. 

## Design Goals

* **Simple**: Offer easy use for the user 
* **Understandable**: As this was a learning project I wanted others to be able to understand how the program is structured

## Installation

To add to project 

```bash
cargo add nhl_stats
```

## Usage

```rust
use nhl_stats::nhl_client::Puck;
     #[tokio::main]
     async fn main() {
         let client = Puck::new();
         let result = client.get_player_career_stats("8478402").await;
         assert!(result.is_ok());
     }
```

## License

[MIT](https://choosealicense.com/licenses/mit/)
