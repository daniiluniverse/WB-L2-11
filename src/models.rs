use chrono::{NaiveDateTime, TimeZone};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub result: Option<String>,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub id: i32,         // Или другой тип, если это нужно
    pub user_id: i32,   // Добавьте это поле
    #[serde(serialize_with = "serialize_naive_date", deserialize_with = "deserialize_naive_date")]
    pub date: NaiveDateTime,
    pub title: String,   // Или другой тип
}

impl Event {
    pub(crate) fn clone(&self) -> Event {
        todo!()
    }
}

fn serialize_naive_date<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&date.format("%Y-%m-%d %H:%M:%S").to_string())
}

fn deserialize_naive_date<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").map_err(serde::de::Error::custom)
}
