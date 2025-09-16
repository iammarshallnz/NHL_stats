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

