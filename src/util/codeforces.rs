use crate::structure::contest::{ContestPhase, Contest, ContestObject};

pub async fn request_incomming_contests_codeforces() -> Vec<Contest> {
    let mut resp = reqwest::get("https://codeforces.com/api/contest.list")
        .await.unwrap()
        .json::<ContestObject>()
        .await.unwrap();

    resp.result = resp.result.into_iter().filter(|i| { i.phase == ContestPhase::BEFORE }).collect();

    resp.result
}