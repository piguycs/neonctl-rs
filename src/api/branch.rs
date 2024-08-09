use serde::{Deserialize, Serialize};

use super::Endpoint;
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
                let projects = self.get_project_list()?.projects;

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
