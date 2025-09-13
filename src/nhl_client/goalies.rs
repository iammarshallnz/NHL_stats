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

    // Deserialize into IndexMap to preserve insertion order
    let obj: IndexMap<String, Value> = serde_json::from_str(&incoming_text).unwrap();
    if client.print_keys{
        for key in obj.keys() {
                println!("{} :: ", key);
            } 
    }
    println!("{:#?}", obj);

    Ok(obj)
}

/// GET current goalie stats leaders for a specific season and game type, optional catagory's
pub async fn current_goalie_stats_season_game_type(client : &Puck,season : &str, game_type : &str, catagory : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/goalie-stats-leaders/{season}/{game_type}?categories={catagory}&limit={}", client.api_limit);
    println!("{}", request_url);
    
    let response = client.get_client.get(request_url).send().await?;
    let incoming_text = response.text().await.unwrap();

    // Deserialize into IndexMap to preserve insertion order
    let obj: IndexMap<String, Value> = serde_json::from_str(&incoming_text).unwrap();
    if client.print_keys{
        for key in obj.keys() {
                println!("{} :: ", key);
            } 
    }
    println!("{:#?}", obj);

    Ok(obj)
}

