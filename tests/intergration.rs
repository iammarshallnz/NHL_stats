


    use nhl_data::Puck;
    
    #[tokio::test]
    async fn test_get_player_career_stats() {
        let client = Puck::new();
        let result = client.get_player_career_stats("8478402").await;
        assert!(result.is_ok());
    }
    


#[tokio::test]
async fn test_get_player_game_log() {
    let client = Puck::new();
    let result = client.get_player_game_log("8478402", "20232024", "2").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_player_game_log_now() {
    let client = Puck::new();
    let result = client.get_player_game_log_now("8478402").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_player_spotlight() {
    let client = Puck::new();
    let result = client.get_player_spotlight().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_current_skater_stats() {
    let client = Puck::new();
    let result = client.get_current_skater_stats("goals").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_current_skater_stats_season_game_type() {
    let client = Puck::new();
    let result = client.get_current_skater_stats_season_game_type("20242025", "2", "").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_current_goalie_stats() {
    let client = Puck::new();
    let result = client.get_current_goalie_stats("").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_current_goalie_stats_season_game_type() {
    let client = Puck::new();
    let result = client.get_current_goalie_stats_season_game_type("20242025", "2", "").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_team_standings() {
    let client = Puck::new();
    let result = client.get_team_standings().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_team_standings_date() {
    let client = Puck::new();
    let result = client.get_team_standings_date("2023-11-10").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_team_standings_season() {
    let client = Puck::new();
    let result = client.get_team_standings_season().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_club_stats() {
    let client = Puck::new();
    let result = client.get_club_stats("TOR").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_club_stats_season() {
    let client = Puck::new();
    let result = client.get_club_stats_season("TOR").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_club_stats_season_game_type() {
    let client = Puck::new();
    let result = client.get_club_stats_season_game_type("TOR", "20232024", "2").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_club_scores() {
    let client = Puck::new();
    let result = client.get_club_scores("TOR").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_team_roster() {
    let client = Puck::new();
    let result = client.get_team_roster("TOR").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_team_roster_season() {
    let client = Puck::new();
    let result = client.get_team_roster_season("TOR", "20232024").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_team_played_seasons() {
    let client = Puck::new();
    let result = client.get_team_played_seasons("TOR").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_team_prospects() {
    let client = Puck::new();
    let result = client.get_team_prospects("TOR").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_scores_now() {
    let client = Puck::new();
    let result = client.get_scores_now().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_scores_date() {
    let client = Puck::new();
    let result = client.get_scores_date("2023-11-10").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_scoreboard() {
    let client = Puck::new();
    let result = client.get_scoreboard().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_play_by_play() {
    let client = Puck::new();
    let result = client.get_play_by_play("2023020001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_landing_page() {
    let client = Puck::new();
    let result = client.get_landing_page("2023020001").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_game_odds() {
    let client = Puck::new();
    let result = client.get_game_odds("US").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_team_schedule_now() {
    let client = Puck::new();
    let result = client.get_team_schedule_now("TOR").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_team_schedule_season() {
    let client = Puck::new();
    let result = client.get_team_schedule_season("TOR", "20232024").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_team_schedule_now_month() {
    let client = Puck::new();
    let result = client.get_team_schedule_now_month("TOR").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_team_schedule_at_month() {
    let client = Puck::new();
    let result = client.get_team_schedule_at_month("TOR", "2023-11").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_team_schedule_at_week() {
    let client = Puck::new();
    let result = client.get_team_schedule_at_week("TOR", "2023-11-10").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_team_schedule_now_week() {
    let client = Puck::new();
    let result = client.get_team_schedule_now_week("TOR").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_schedule_now() {
    let client = Puck::new();
    let result = client.get_schedule_now().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_schedule_date() {
    let client = Puck::new();
    let result = client.get_schedule_date("2023-11-10").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_schedule_calander() {
    let client = Puck::new();
    let result = client.get_schedule_calendar().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_schedule_calander_date() {
    let client = Puck::new();
    let result = client.get_schedule_calendar_date("2023-11-10").await;
    assert!(result.is_ok());
}







