// These require the `serde` dependency.
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClubStats {
    pub season: i64,
    pub game_types: Vec<i64>,
}

pub async fn gametypes_per_season_directory_by_team(client : reqwest::Client, club : &str) -> Result<(), reqwest::Error> {
    /* Gets all game types played by a team throughout their history.

        A dictionary containing game types for each season the team has existed in the league.

        Args:
            team_abbr (str): The 3 letter abbreviation of the team (e.g., BUF, TOR)

        Returns:
            dict: A mapping of seasons to game types played by the team

        Example:
            https://api-web.nhle.com/v1/club-stats-season/TOR

            [
                {'season': 20242025, 'gameTypes': [2]},
                {'season': 20232024, 'gameTypes': [2, 3]},
                {'season': 20222023, 'gameTypes': [2, 3]},
                {'season': 20212022, 'gameTypes': [2, 3]},
             ...
             ]
    */
    let request_url = format!("https://api-web.nhle.com/v1/club-stats-season/{club}");
    println!("{}", request_url);
    
    
    
    let response = client.get(request_url).send().await?;
    
    
    let root: Vec<ClubStats> = response.json().await?;
    println!("{:?}", root);
    // ...existing code...


    Ok(())
}




pub mod nhl_client;




#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let input = if let Some(input) = std::env::args().nth(1) {
        input
    } else {
        println!("No input provided");
        return Ok(());
    };
    let client = nhl_client::Puck::new();
    
    if input == "1" {
        //gametypes_per_season_directory_by_team(client.open(), "TOR").await?;
        // client.get_player_career_stats("8478402").await?;
        // client.get_player_game_log("8478402", "20232024", "2").await?;
        //client.get_player_game_log_now("8478402").await?;
        // client.get_current_skater_stats("goals").await?;
        //client.get_current_skater_stats_season_game_type("20242025", "2", "").await?;
        client.get_current_goalie_stats("").await?;
    }
    
    
    Ok(())
}