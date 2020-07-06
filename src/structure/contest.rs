use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, PartialEq, Serialize, Clone, Copy)]
pub enum ContestType {
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
    name: String,
    r#type: ContestType,
    phase: ContestPhase,
    frozen: bool,
    duration_seconds: i64,
    url: String,
    #[serde(default)]
    pub start_time_seconds: i64,
    #[serde(default)]
    description: String,
    #[serde(default)]
    difficulty: i64,

    // For ICPC contests
    #[serde(default)]
    icpc_region: String,
    #[serde(default)]
    country: String,
    #[serde(default)]
    city: String,
}

impl Contest {
    pub fn new(name: String, typ: ContestType, phase: ContestPhase, frozen: bool, 
            duration_seconds: i64, url: String, start_time_seconds: i64,
            description: String, difficulty: i64, 
            icpc_region: String, country: String, city: String) -> Contest {
        Contest {
            name: name,
            r#type: typ,
            phase: phase,
            frozen: frozen,
            duration_seconds: duration_seconds,
            url: url,
            start_time_seconds: start_time_seconds,
            description: description,
            difficulty: difficulty,
            icpc_region: icpc_region,
            country: country,
            city: city
        }
    }
}