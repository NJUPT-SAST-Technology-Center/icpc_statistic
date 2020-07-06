use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, PartialEq, Serialize, Clone, Copy)]
enum ContestType {
    CF, IOI, ICPC
}

#[derive(Deserialize, Debug, PartialEq, Serialize, Clone, Copy)]
pub enum ContestPhase {
    BEFORE, 
    CODING, 
    #[serde(rename = "PENDING_SYSTEM_TEST")]
    PENDINGSYSTEMTEST, 
    #[serde(rename = "SYSTEM_TEST")]
    SYSTEMTEST, 
    FINISHED
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Contest {
    id: i64,
    name: String,
    r#type: ContestType,
    pub phase: ContestPhase,
    frozen: bool,
    #[serde(rename = "durationSeconds")]
    duration_seconds: i64,
    #[serde(default, rename = "startTimeSeconds")]
    start_time_seconds: i64,
    #[serde(default, rename = "relativeTimeSecond")]
    relative_time_second: i64,
    #[serde(default, rename = "preparedBy")]
    prepared_by: String,
    #[serde(default, rename = "websiteUrl")]
    website_url: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    difficulty: i64,
    #[serde(default)]
    kind: String,
    #[serde(default, rename = "icpcRegion")]
    icpc_region: String,
    #[serde(default)]
    country: String,
    #[serde(default)]
    city: String,
    #[serde(default)]
    season: String,
}

#[derive(Deserialize, Debug)]
pub struct ContestObject {
    status: String,
    pub result: Vec<Contest>
}