pub mod player;
pub mod skaters;
use serde_json::Value;
use indexmap::IndexMap;

pub struct Puck{
    get_client : reqwest::Client,
    club_name : String,
    print_keys : bool
}

impl Puck {
    pub fn new() -> Puck{
        Puck {
            get_client: reqwest::Client::new(),
            club_name: "TOR".to_string(),
            print_keys : true
        }
    }

    pub fn open(self) -> reqwest::Client{
        self.get_client

    }
    //
    // Skaters
    //
    pub async fn get_player_career_stats(&self, id: &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(player::player_career_stats(&self, id).await?)
    }
    pub async fn get_player_game_log(&self, id: &str, season : &str, game_type : &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(player::player_game_log(&self, id, season, game_type).await?)
    }
    pub async fn get_player_game_log_now(&self, id: &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(player::player_game_log_now(&self, id).await?)
    }


    ///
    /// 
    ///  Skaters
    /// 
    /// 
    pub async fn get_current_skater_stats(&self, catagory: &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(skaters::current_skater_stats(&self, catagory).await?)
    }
    
}