pub use serde_json::Value;
pub use indexmap::IndexMap;

use crate::nhl_client::Puck;


pub async fn current_skater_stats(client : &Puck, catagory : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
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

    let request_url = format!("https://api-web.nhle.com/v1/skater-stats-leaders/current?categories={catagory}");
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
