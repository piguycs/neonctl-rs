use serde::{Deserialize, Serialize};

use super::{Api, Endpoint};
use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct AuthAccount {
    pub email: String,
    pub image: String,
    pub login: String,
    pub name: String,
    pub provider: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub active_seconds_limit: u32,
    // billing_account not relevant
    pub auth_accounts: Vec<AuthAccount>,
    pub email: String,
    pub id: String,
    pub image: String,
    pub login: String,
    pub name: String,
    pub last_name: String,
    pub projects_limit: u32,
    pub branches_limit: u32,
    pub max_autoscaling_limit: f32,
    pub plan: String,
}

impl Api {
    pub fn get_user_details(&self) -> Result<User> {
        let res: User = self.call(ureq::get(&Endpoint::Me.to_string()))?;

        Ok(res)
    }
}

#[test]
fn deserialise() {
    let key = std::env::var("API_KEY").unwrap();
    let res = ureq::get(&Endpoint::Me.to_string())
        .set("Authorization", &format!("Bearer {}", key))
        .call();

    let _: User = res.unwrap().into_json().unwrap();
}
