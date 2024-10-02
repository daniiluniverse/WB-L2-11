use chrono::NaiveDate;
use crate::models::{Event, ApiResponse};
use serde_json::json;

pub fn serialize_event(event: &Event) -> serde_json::Value {
    json!({
        "id": event.id,
        "user_id": event.user_id,
        "date": event.date.to_string(),
        "title": event.title,
    })
}

pub fn parse_event_params(body: &str) -> Result<Event, String> {
    let params: Vec<&str> = body.split('&').collect();
    let mut user_id = 0;
    let mut date = String::new();
    let mut title = String::new();

    for param in params {
        let pair: Vec<&str> = param.split('=').collect();
        if pair.len() != 2 {
            return Err("Invalid parameters".to_string());
        }
        match pair[0] {
            "user_id" => user_id = pair[1].parse().map_err(|_| "Invalid user_id".to_string())?,
            "date" => date = pair[1].to_string(),
            "title" => title = pair[1].to_string(),
            _ => return Err("Unknown parameter".to_string()),
        }
    }

    let date_time = NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|_| "Invalid date format".to_string())?;
    Ok(Event { id: 0, user_id, date: date_time.and_hms(0, 0, 0), title })
}
