pub use serde_json::Value;
pub use indexmap::IndexMap;

use crate::nhl_client::Puck;

/// #### Get Team Roster As of Now
/// - **Description**: Retrieve the roster for a specific team as of the current moment.
/// - **Endpoint**: `/v1/roster/{team}/current`
/// - **Parameters**:
///   - `team` (string) - Three-letter team code
pub async fn team_roster(client : &Puck, team : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/roster/{team}/current");
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
/// #### Get Team Roster As of selected season
/// - 
pub async fn team_roster_season(client : &Puck, team : &str, season : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/roster/{team}/{season}");
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

/// #### Get every season played by team
/// - **return a list of all of the seasons that the team played.**
pub async fn team_played_seasons(client : &Puck, team : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/roster-season/{team}");
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

/// #### Get prospects for a specific team.
/// - **return a list of all of the seasons that the team played.**
pub async fn team_prospects(client : &Puck, team : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/prospects/{team}");
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

