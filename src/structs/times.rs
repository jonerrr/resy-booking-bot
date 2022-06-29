use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

mod resy_date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VenueResponse {
    pub results: VenueResults,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VenueResults {
    pub venues: Vec<VenueDetails>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VenueDetails {
    pub slots: Vec<VenueSlots>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VenueSlots {
    pub config: VenueConfig,
    pub date: VenueDate,
    pub template: VenueTemplate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VenueConfig {
    pub id: i32,
    pub token: String,
    #[serde(rename = "type")]
    pub location: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VenueDate {
    #[serde(with = "resy_date_format")]
    pub end: NaiveDateTime,
    #[serde(with = "resy_date_format")]
    pub start: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VenueTemplate {
    pub id: i32,
}
