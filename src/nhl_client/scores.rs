pub use serde_json::Value;
pub use indexmap::IndexMap;

use crate::nhl_client::Puck;



/// #### Get Daily Scores As of Now
/// - ** Retrieve daily scores as of the current moment.**

pub async fn scores_now(client : &Puck) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/score/now");
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

/// #### Get Daily Scores by Date
/// - ** Retrieve daily scores for a specific date.**
/// - **'date' (string) - Date in YYYY-MM-DD format**
pub async fn scores_date(client : &Puck, date : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/score/{date}");
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

/// #### Get Scoreboard
/// - ** Retrieve the overall scoreboard as of the current moment.**
pub async fn scoreboard(client : &Puck) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/scoreboard/now");
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


