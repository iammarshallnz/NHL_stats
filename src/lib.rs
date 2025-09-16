pub mod nhl_client;

pub struct Puck{
    get_client : reqwest::Client,
    print_keys : bool,
    api_limit : i32,
}
