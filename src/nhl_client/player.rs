pub use serde_json::Value;
pub use indexmap::IndexMap;

use crate::nhl_client::Puck;

pub async fn player_career_stats(client : &Puck, player_id : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    // example >>>>>>>
    // player_id = 8478402
    // playerId :: 8478402
    // isActive :: true
    // currentTeamId :: 22
    // currentTeamAbbrev :: "EDM"
    // fullTeamName :: {"default":"Edmonton Oilers","fr":"Oilers d'Edmonton"}
    // teamCommonName :: {"default":"Oilers"}
    // teamPlaceNameWithPreposition :: {"default":"Edmonton","fr":"d'Edmonton"}
    // firstName :: {"default":"Connor"}
    // lastName :: {"default":"McDavid"}

    let request_url = format!("https://api-web.nhle.com/v1/player/{player_id}/landing");
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

    Ok(obj)
}



pub async fn player_game_log(client : &Puck, player_id : &str, season : &str, game_type : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    // example >>>>>>>
    // player_id = 8478402
    // season = 20232024
    // game_type = 2 (regular , 3 is play off)
    // 
    //
    // seasonId ::
    // gameTypeId ::
    // playerStatsSeasons ::
    // gameLog ::

    let request_url = format!("https://api-web.nhle.com/v1/player/{player_id}/game-log/{season}/{game_type}");
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
    

    Ok(obj)
}

pub async fn player_game_log_now(client : &Puck, player_id : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    // example >>>>>>>
    // player_id = 8478402
    // 
    //
    // seasonId ::
    // gameTypeId ::
    // playerStatsSeasons ::
    // gameLog ::

    let request_url = format!("https://api-web.nhle.com/v1/player/{player_id}/game-log/now");
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
    

    Ok(obj)
}


pub async fn player_spotlight(client : &Puck) -> Result<IndexMap<String, Value>, reqwest::Error> {
    // example >>>>>>>
    // player_id = 8478402
    // 
    //
    // seasonId ::
    // gameTypeId ::
    // playerStatsSeasons ::
    // gameLog ::

    let request_url = format!("https://api-web.nhle.com/v1/player-spotlight");
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
    

    Ok(obj)
}

