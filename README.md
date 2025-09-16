# nhl_data

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
The project is almost a 1 to 1 of [Zmalski's nhl api reference guide](https://github.com/Zmalski/NHL-API-Reference) so thanks to him for documenting all of this.
There is some stuff missing from this guide, namely: networking info, stream info and playoff info. 
## Issues

Currently I do not handle errors, this is the next thing for me to implement. Just try not to call functions with incorrect parameters :) 


## Design Goals

* **Simple**: Offer easy use for the user 
* **Understandable**: As this was a learning project I wanted others to be able to understand how the program is structured

## Installation

To add to project 

```bash
cargo add nhl_data
```

## Usage

```rust
use nhl_data::Puck;

    #[tokio::main]
    async fn main() {
        let client = Puck::new();
        let mc_david = "8478402";
        let result = client.get_player_career_stats(mc_david).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        let v = result.get("awards").unwrap();
        if let Some(awards) = v.as_array() {
        for award in awards {
            // Get the trophy name
            if let Some(trophy) = award.get("trophy")
                                       .and_then(|t| t.get("default"))
                                       .and_then(|t| t.as_str()) {
                println!("🏆 {}", trophy);
            }

            // Get the seasons list
            if let Some(seasons) = award.get("seasons").and_then(|s| s.as_array()) {
                for season in seasons {
                    if let Some(season_id) = season.get("seasonId").and_then(|id| id.as_u64()) {
                        println!("   Season: {}", season_id);
                    }
                }
            }
        }
    }      
    }
```

## License

[MIT](https://choosealicense.com/licenses/mit/)
