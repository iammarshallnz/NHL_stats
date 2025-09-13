pub use serde_json::Value;
pub use indexmap::IndexMap;

use crate::nhl_client::Puck;

/// Get current statistics for a specific club.
/// Parameters : CLUB      (TOR)
pub async fn club_stats(client : &Puck, club : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/club-stats/{club}/now");
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

/// Get an overview of the stats for each season for a specific club. Seems to only indicate the gametypes played in each season.
/// 
/// Parameters : CLUB      (TOR)
pub async fn club_stats_season(client : &Puck, club : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/club-stats-season/{club}");
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

/// Get the stats for a specific team, season, and game type.
/// Parameters : CLUB      (TOR)
///            : Season    (20232024)
///            : Game_type (2 for regular season, 3 for playoffs)
pub async fn club_stats_season_game_type(client : &Puck, club : &str, season : &str, game_type : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/club-stats/{club}/{season}/{game_type}");
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
/// Retrieve the scoreboard for a specific team as of the current moment.
/// Parameters : CLUB      (TOR)
pub async fn club_scores(client : &Puck, club : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/scoreboard/{club}/now");
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