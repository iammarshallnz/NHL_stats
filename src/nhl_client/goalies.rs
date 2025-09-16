pub use serde_json::Value;
pub use indexmap::IndexMap;

use crate::nhl_client::Puck;



/// GET current goalie stats leaders, optional catagory's
pub async fn current_goalie_stats(
    client : &Puck,
    catagory : &str
) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/goalie-stats-leaders/current?categories={catagory}&limit={}", client.api_limit);
    println!("{}", request_url);
    
    let response = client.get_client.get(request_url).send().await?;
    let incoming_text = response.text().await.unwrap();

    // Parse as Value first
    let value: Value = serde_json::from_str(&incoming_text).unwrap();
    let obj: IndexMap<String, Value> = match value {
        Value::Object(map) => map.into_iter().collect(),
        Value::Array(arr) => arr.into_iter().enumerate().map(|(i, v)| (i.to_string(), v)).collect(),
        _ => IndexMap::new(),
    };

    if client.print_keys {
        for key in obj.keys() {
            println!("{} :: ", key);
        }
    }
    

    Ok(obj)
}

/// GET current goalie stats leaders for a specific season and game type, optional catagory's
pub async fn current_goalie_stats_season_game_type(client : &Puck,season : &str, game_type : &str, catagory : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/goalie-stats-leaders/{season}/{game_type}?categories={catagory}&limit={}", client.api_limit);
    println!("{}", request_url);
    
    let response = client.get_client.get(request_url).send().await?;
    let incoming_text = response.text().await.unwrap();

    // Parse as Value first
    let value: Value = serde_json::from_str(&incoming_text).unwrap();
    let obj: IndexMap<String, Value> = match value {
        Value::Object(map) => map.into_iter().collect(),
        Value::Array(arr) => arr.into_iter().enumerate().map(|(i, v)| (i.to_string(), v)).collect(),
        _ => IndexMap::new(),
    };

    if client.print_keys {
        for key in obj.keys() {
            println!("{} :: ", key);
        }
    }
    

    Ok(obj)
}

