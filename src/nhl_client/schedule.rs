pub use serde_json::Value;
pub use indexmap::IndexMap;

use crate::nhl_client::Puck;

/// Get Team Season Schedule As of Now
/// 
/// # Arguments
/// * `client` - Reference to Puck client
/// * `team` - Team code (e.g., "TOR")
/// 
/// # Returns
/// * Team's season schedule as of now
pub async fn team_schedule_now(client : &Puck, team : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/club-schedule-season/{team}/now");
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

/// Get Team Season Schedule
/// 
/// # Arguments
/// * `client` - Reference to Puck client
/// * `team` - Team code
/// * `season` - Season string (e.g., "20232024")
/// 
/// # Returns
/// * Team's schedule for the specified season
pub async fn team_schedule_season(client : &Puck, team : &str, season : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/club-schedule-season/{team}/{season}");
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

/// Get Month Schedule As of Now
/// 
/// # Arguments
/// * `client` - Reference to Puck client
/// * `team` - Team code
/// 
/// # Returns
/// * Team's monthly schedule as of now
pub async fn team_schedule_now_month(client : &Puck, team : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/club-schedule/{team}/month/now");
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

/// Get Month Schedule
/// 
/// # Arguments
/// * `client` - Reference to Puck client
/// * `team` - Team code
/// * `month` - Date in YYYY-MMformat
/// 
/// # Returns
/// * Team's monthly schedule for the given date
pub async fn team_schedule_at_month(client : &Puck, team : &str, month : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/club-schedule/{team}/month/{month}");
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

/// Get Week Schedule
/// 
/// # Arguments
/// * `client` - Reference to Puck client
/// * `team` - Team code
/// * `date` - Date in YYYY-MM-DD format
/// 
/// # Returns
/// * Team's weekly schedule for the given date
pub async fn team_schedule_at_week(client : &Puck, team : &str, date : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/club-schedule/{team}/week/{date}");
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

/// Get Week Schedule As of Now
/// 
/// # Arguments
/// * `client` - Reference to Puck client
/// * `team` - Team code
/// 
/// # Returns
/// * Team's weekly schedule as of now
pub async fn team_schedule_now_week(client : &Puck, team : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/club-schedule/{team}/week/now");
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

/// Get Current Schedule
/// 
/// # Arguments
/// * `client` - Reference to Puck client
/// 
/// # Returns
/// * Current NHL schedule
pub async fn schedule_now(client : &Puck) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/schedule/now");
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

/// Get Schedule by Date
/// 
/// # Arguments
/// * `client` - Reference to Puck client
/// * `date` - Date in YYYY-MM-DD format
/// 
/// # Returns
/// * NHL schedule for the given date
pub async fn schedule_date(client : &Puck, date : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/schedule/{date}");
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

/// Get Schedule Calendar for Now
/// 
/// # Arguments
/// * `client` - Reference to Puck client
/// 
/// # Returns
/// * NHL schedule calendar as of now
pub async fn schedule_calendar(client : &Puck) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/schedule-calendar/now");
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

/// Get Schedule Calendar for a Specific Date
/// 
/// # Arguments
/// * `client` - Reference to Puck client
/// * `date` - Date in YYYY-MM-DD format
/// 
/// # Returns
/// * NHL schedule calendar for the given date
pub async fn schedule_calendar_date(client : &Puck, date : &str) -> Result<IndexMap<String, Value>, reqwest::Error> {
    
    

    let request_url = format!("https://api-web.nhle.com/v1/schedule-calendar/{date}");
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