use serde::{Deserialize, Serialize};

use super::*;
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub current_state: String,
    pub logical_size: Option<u64>,
    pub creation_source: String,
    pub primary: bool,
    pub default: bool,
    pub protected: bool,
    pub cpu_used_sec: u64,
    pub compute_time_seconds: u64,
    pub active_time_seconds: u64,
    pub written_data_bytes: u64,
    pub data_transfer_bytes: u64,
    pub created_at: String,
    pub updated_at: String,
}

impl Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Root {
    branches: Vec<Branch>,
}

impl Api {
    pub fn get_branch_list(&self, id: Option<String>) -> Result<Vec<Branch>> {
        let branches = match id {
            Some(id) => {
                self.call::<Root>(ureq::get(&Endpoint::BranchList(id).to_string()))?
                    .branches
            }
            None => {
                let projects = self.get_project_list()?;

                projects
                    .into_iter()
                    .map(|project| {
                        self.call::<Root>(ureq::get(&Endpoint::BranchList(project.id).to_string()))
                            .expect("invalid json parsing")
                    })
                    .flat_map(|res| res.branches)
                    .collect()
            }
        };

        Ok(branches)
    }
}

#[test]
fn deserialise() {
    let key = std::env::var("API_KEY").unwrap();
    let res = ureq::get(&Endpoint::BranchList("invalid".to_string()).to_string())
        .set("Authorization", &format!("Bearer {}", key))
        .call();

    // TODO: first create a branch, then delete it
    // so for now, we just check if invalid id returns 404
    // very bad test, the things I do because project limit is 1 :(
    // maybe I can self host neondb for testing? but idk if the api comes with it or just psql
    assert!(res.is_err());
}
