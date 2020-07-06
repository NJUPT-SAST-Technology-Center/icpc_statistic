use crate::structure::contest::{ContestPhase, Contest, ContestType};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct ContestObject {
    status: String,
    result: Vec<CFContest>
}

fn default_int() -> i64 {
    0
}

fn default_string() -> String {
    "".to_string()
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct CFContest {
    id: i64,
    name: String,
    r#type: ContestType,
    pub phase: ContestPhase,
    frozen: bool,
    #[serde(rename = "durationSeconds")]
    duration_seconds: i64,
    #[serde(default = "default_int", rename = "startTimeSeconds")]
    start_time_seconds: i64,
    #[serde(default, rename = "relativeTimeSecond")]
    relative_time_second: i64,
    #[serde(default, rename = "preparedBy")]
    prepared_by: String,
    #[serde(default, rename = "websiteUrl")]
    website_url: String,
    #[serde(default = "default_string")]
    description: String,
    #[serde(default = "default_int")]
    difficulty: i64,
    #[serde(default)]
    kind: String,
    #[serde(default = "default_string", rename = "icpcRegion")]
    icpc_region: String,
    #[serde(default = "default_string")]
    country: String,
    #[serde(default = "default_string")]
    city: String,
    #[serde(default)]
    season: String,
}

pub async fn request_incomming_contests_codeforces() -> Vec<Contest> {
    let resp = reqwest::get("https://codeforces.com/api/contest.list")
        .await.unwrap()
        .json::<ContestObject>()
        .await.unwrap();

    resp.result.into_iter().filter(|i| { i.phase == ContestPhase::BEFORE })
    .map(|o| {
        Contest::new(o.name, o.r#type, o.phase, o.frozen, o.duration_seconds, 
            "https://codeforces.com/contest/".to_string() + &o.id.to_string(), 
            o.start_time_seconds, o.description, o.difficulty, 
            o.icpc_region, o.country, o.city)
    }).collect::<Vec<Contest>>()
}