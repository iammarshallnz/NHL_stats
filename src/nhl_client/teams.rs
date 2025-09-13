pub use serde_json::Value;
pub use indexmap::IndexMap;

use crate::nhl_client::Puck;

/// Get current team standings 
pub async fn team_standings(client : &Puck) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/standings/now");
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
/// get standings for specific date; date format : 2023-11-10
pub async fn team_standings_date(client : &Puck, date : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/standings/{date}");
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


/// get information for each season's standings
pub async fn team_standings_season(client : &Puck) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/standings-season");
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