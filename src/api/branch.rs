use serde::{Deserialize, Serialize};

use super::*;
use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Branch {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub current_state: String,
    pub logical_size: u32,
    pub created_at: String,
    pub updated_at: String,
    pub data_transfer_bytes: u64,
    pub written_data_bytes: u64,
    pub compute_time_seconds: u64,
    pub active_time_seconds: u64,
    pub cpu_used_sec: u64,
    pub primary: bool,
    pub default: bool,
    pub protected: bool,
    pub creation_source: String,
    pub parent_id: Option<String>,
    pub parent_lsn: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Root {
    branches: Vec<Branch>,
}

impl Api {
    pub fn get_branch_list(&self, id: String) -> Result<Vec<Branch>> {
        let branches: Root = self.call(ureq::get(&Endpoint::BranchList(id).to_string()))?;

        Ok(branches.branches)
    }
}
