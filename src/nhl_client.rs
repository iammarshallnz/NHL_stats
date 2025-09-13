pub mod player;
pub mod skaters;
pub mod goalies;
use serde_json::Value;
use indexmap::IndexMap;

pub struct Puck{
    get_client : reqwest::Client,
    print_keys : bool,
    api_limit : i32,
}

impl Puck {
    pub fn new() -> Puck{
        Puck {
            get_client: reqwest::Client::new(),
            print_keys : true,
            api_limit : 5,
        }
    }

    pub fn open(self) -> reqwest::Client{
        self.get_client

    }
}
    ///
    /// Players
    /// 
impl Puck {
    pub async fn get_player_career_stats(&self, id: &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(player::player_career_stats(&self, id).await?)
    }
    pub async fn get_player_game_log(&self, id: &str, season : &str, game_type : &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(player::player_game_log(&self, id, season, game_type).await?)
    }
    pub async fn get_player_game_log_now(&self, id: &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(player::player_game_log_now(&self, id).await?)
    }
}
    

    /// 
    ///  Skaters
    /// 
impl Puck {
     pub async fn get_current_skater_stats(&self, catagory: &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(skaters::current_skater_stats(&self, catagory).await?)
    }
    pub async fn get_current_skater_stats_season_game_type(&self, season : &str, game_type :&str, catagory: &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(skaters::current_skater_stats_season_game_type(&self, season, game_type, catagory).await?)
    }
}

    ///
    /// Goalies
    /// 
impl Puck {
    pub async fn get_current_goalie_stats(&self,catagory: &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(goalies::current_goalie_stats(&self, catagory).await?)
    }
    pub async fn get_current_goalie_stats_season_game_type(&self, season : &str, game_type :&str, catagory: &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(goalies::current_goalie_stats_season_game_type(&self, season, game_type, catagory).await?)
    }
}

    
   

    
    
    
