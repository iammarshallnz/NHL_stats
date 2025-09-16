mod player;
mod skaters;
mod goalies;
mod teams;
mod clubs;
mod roster;
mod schedule;
mod scores;
mod game;


use serde_json::Value;
use indexmap::IndexMap;
use crate::Puck;


/// # Puck util's
/// 
impl Puck {
    /// Create a new Puck client.
    ///
    /// # Example
    /// 
    /// ```
    /// use nhl_data::Puck;
    /// let client = Puck::new();
    /// ```
    pub fn new() -> Puck{
        Puck {
            get_client: reqwest::Client::new(),
            print_keys : false,
            api_limit : 5,
        }
    }

    /// Set whether to print keys from API responses.
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// let client = Puck::new().set_print(true);
    /// ```
    pub fn set_print(mut self, setting : bool) -> Puck{
        self.print_keys = setting;
        self
    }
    /// Get the underlying reqwest client.
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// let client = Puck::new();
    /// let reqwest_client = client.open();
    /// ```
    pub fn open(self) -> reqwest::Client{
        self.get_client

    }
}
/// # Players
///
// Methods for querying player-related NHL stats.
impl Puck {
    /// Get career stats for a player.
    ///
    /// # Arguments
    /// * `id` - Player ID
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_player_career_stats("8478402").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_player_career_stats(&self, id: &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(player::player_career_stats(&self, id).await?)
    }
    /// Get game log for a player for a season and game type.
    ///
    /// # Arguments
    /// * `id` - Player ID
    /// * `season` - Season string
    /// * `game_type` - Game type (2=regular, 3=playoff)
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_player_game_log("8478402", "20232024", "2").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_player_game_log(&self, id: &str, season : &str, game_type : &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(player::player_game_log(&self, id, season, game_type).await?)
    }
    /// Get current game log for a player.
    ///
    /// # Arguments
    /// * `id` - Player ID
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_player_game_log_now("8478402").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_player_game_log_now(&self, id: &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(player::player_game_log_now(&self, id).await?)
    }
    /// Get player spotlight data.
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_player_spotlight().await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_player_spotlight(&self) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(player::player_spotlight(&self).await?)
    }
}
    

/// # Skaters
///
// Methods for querying skater-related NHL stats.
impl Puck {
    /// Get current skater stats leaders.
    ///
    /// # Arguments
    /// * `category` - Stat category (can be empty)
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_current_skater_stats("goals").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_current_skater_stats(&self, category: &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(skaters::current_skater_stats(&self, category).await?)
    }
    /// Get skater stats leaders for a season and game type.
    ///
    /// # Arguments
    /// * `season` - Season string
    /// * `game_type` - Game type
    /// * `category` - Stat category
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_current_skater_stats_season_game_type("20242025", "2", "").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_current_skater_stats_season_game_type(&self, season : &str, game_type :&str, category: &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(skaters::current_skater_stats_season_game_type(&self, season, game_type, category).await?)
    }
}

/// # Goalies
///
// Methods for querying goalie-related NHL stats.
impl Puck {
    /// Get current goalie stats leaders.
    ///
    /// # Arguments
    /// * `category` - Stat category (can be empty)
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_current_goalie_stats("").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_current_goalie_stats(&self, category: &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(goalies::current_goalie_stats(&self, category).await?)
    }
    /// Get goalie stats leaders for a season and game type.
    ///
    /// # Arguments
    /// * `season` - Season string
    /// * `game_type` - Game type
    /// * `category` - Stat category
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_current_goalie_stats_season_game_type("20242025", "2", "").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_current_goalie_stats_season_game_type(&self, season: &str, game_type: &str, category: &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(goalies::current_goalie_stats_season_game_type(&self, season, game_type, category).await?)
    }
}
/// # Teams
///
// Methods for querying team-related NHL stats.
impl Puck {
    /// Get current team standings.
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_team_standings().await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_team_standings(&self) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(teams::team_standings(&self).await?)
    }
    /// Get team standings for a specific date.
    ///
    /// # Arguments
    /// * `date` - Date in YYYY-MM-DD format
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_team_standings_date("2023-11-10").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_team_standings_date(&self, date : &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(teams::team_standings_date(&self, date).await?)
    }
    /// Get team standings for each season.
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_team_standings_season().await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_team_standings_season(&self) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(teams::team_standings_season(&self).await?)
    }
}
/// # Clubs
///
// Methods for querying club-related NHL stats.
impl Puck {
    /// Get current stats for a club.
    ///
    /// # Arguments
    /// * `team` - Team code
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_club_stats("TOR").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_club_stats(&self, team : &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(clubs::club_stats(&self, team).await?)
    }
    /// Get club stats for each season.
    ///
    /// # Arguments
    /// * `team` - Team code
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_club_stats_season("TOR").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_club_stats_season(&self, team : &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(clubs::club_stats_season(&self, team).await?)
    }
    /// Get club stats for a season and game type.
    ///
    /// # Arguments
    /// * `team` - Team code
    /// * `season` - Season string
    /// * `game_type` - Game type
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_club_stats_season_game_type("TOR", "20232024", "2").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_club_stats_season_game_type(&self, team : &str, season : &str, game_type : &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(clubs::club_stats_season_game_type(&self, team, season, game_type).await?)
    }
    /// Get club scoreboard as of now.
    ///
    /// # Arguments
    /// * `team` - Team code
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_club_scores("TOR").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_club_scores(&self, team : &str) -> Result<IndexMap<String, Value>, reqwest::Error> { 
        Ok(clubs::club_scores(&self, team).await?)
    }
}
/// # Roster
///
// Methods for querying roster-related NHL stats.
impl Puck {
    /// Get team roster as of now.
    ///
    /// # Arguments
    /// * `team` - Team code
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_team_roster("TOR").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_team_roster(&self, team: &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(roster::team_roster(&self, team).await?)
    }
    /// Get team roster for a specific season.
    ///
    /// # Arguments
    /// * `team` - Team code
    /// * `season` - Season string
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_team_roster_season("TOR", "20232024").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_team_roster_season(&self, team: &str, season: &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(roster::team_roster_season(&self, team, season).await?)
    }
    /// Get all seasons played by a team.
    ///
    /// # Arguments
    /// * `team` - Team code
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_team_played_seasons("TOR").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_team_played_seasons(&self, team: &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(roster::team_played_seasons(&self, team).await?)
    }
    /// Get prospects for a team.
    ///
    /// # Arguments
    /// * `team` - Team code
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_team_prospects("TOR").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_team_prospects(&self, team: &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(roster::team_prospects(&self, team).await?)
    }
}
/// # Scores
///
// Methods for querying scores-related NHL stats.
impl Puck {
    /// Get daily scores as of now.
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_scores_now().await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_scores_now(&self) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(scores::scores_now(&self).await?)
    }
    /// Get daily scores for a specific date.
    ///
    /// # Arguments
    /// * `date` - Date in YYYY-MM-DD format
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_scores_date("2023-11-10").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_scores_date(&self, date: &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(scores::scores_date(&self, date).await?)
    }
    /// Get overall scoreboard as of now.
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_scoreboard().await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_scoreboard(&self) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(scores::scoreboard(&self).await?)
    }
}
/// # Game
///
// Methods for querying game-related NHL stats.
impl Puck {
    /// Get play-by-play information for a game.
    ///
    /// # Arguments
    /// * `game_id` - Game ID
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_play_by_play("2023020001").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_play_by_play(&self, game_id: &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(game::play_by_play(&self, game_id).await?)
    }
    /// Get landing page information for a game.
    ///
    /// # Arguments
    /// * `game_id` - Game ID
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_landing_page("2023020001").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_landing_page(&self, game_id: &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(game::landing_page(&self, game_id).await?)
    }
    /// Get game odds for a country.
    ///
    /// # Arguments
    /// * `country_code` - Country code
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_game_odds("US").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_game_odds(&self, country_code: &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(game::game_odds(&self, country_code).await?)
    }
}
/// # Schedule
///
// Methods for querying schedule-related NHL stats.
impl Puck {
    /// Get team season schedule as of now.
    ///
    /// # Arguments
    /// * `team` - Team code
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_team_schedule_now("TOR").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_team_schedule_now(&self, team: &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(schedule::team_schedule_now(&self, team).await?)
    }
    /// Get team season schedule for a specific season.
    ///
    /// # Arguments
    /// * `team` - Team code
    /// * `season` - Season string
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_team_schedule_season("TOR", "20232024").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_team_schedule_season(&self, team: &str, season: &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(schedule::team_schedule_season(&self, team, season).await?)
    }
    /// Get team monthly schedule as of now.
    ///
    /// # Arguments
    /// * `team` - Team code
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_team_schedule_now_month("TOR").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_team_schedule_now_month(&self, team: &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(schedule::team_schedule_now_month(&self, team).await?)
    }
    /// Get team monthly schedule for a specific date.
    ///
    /// # Arguments
    /// * `team` - Team code
    /// * `month` - Date in YYYY-MM format
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_team_schedule_at_month("TOR", "2023-11").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_team_schedule_at_month(&self, team: &str, month: &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(schedule::team_schedule_at_month(&self, team, month).await?)
    }
    /// Get team weekly schedule for a specific date.
    ///
    /// # Arguments
    /// * `team` - Team code
    /// * `date` - Date in YYYY-MM-DD format
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_team_schedule_at_week("TOR", "2023-11-10").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_team_schedule_at_week(&self, team: &str, date: &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(schedule::team_schedule_at_week(&self, team, date).await?)
    }
    /// Get team weekly schedule as of now.
    ///
    /// # Arguments
    /// * `team` - Team code
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_team_schedule_now_week("TOR").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_team_schedule_now_week(&self, team: &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(schedule::team_schedule_now_week(&self, team).await?)
    }
    /// Get current NHL schedule.
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_schedule_now().await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_schedule_now(&self) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(schedule::schedule_now(&self).await?)
    }
    /// Get NHL schedule for a specific date.
    ///
    /// # Arguments
    /// * `date` - Date in YYYY-MM-DD format
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_schedule_date("2023-11-10").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_schedule_date(&self, date: &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(schedule::schedule_date(&self, date).await?)
    }
    /// Get NHL schedule calendar as of now.
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_schedule_calendar().await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_schedule_calendar(&self) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(schedule::schedule_calendar(&self).await?)
    }
    /// Get NHL schedule calendar for a specific date.
    ///
    /// # Arguments
    /// * `date` - Date in YYYY-MM-DD format
    ///
    /// # Example
    /// ```
    /// use nhl_data::Puck;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Puck::new();
    ///     let result = client.get_schedule_calendar_date("2023-11-10").await;
    ///     assert!(result.is_ok());
    /// }
    /// ```
    pub async fn get_schedule_calendar_date(&self, date: &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
        Ok(schedule::schedule_calendar_date(&self, date).await?)
    }
}



