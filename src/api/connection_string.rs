use serde::{Deserialize, Serialize};

use super::*;
use crate::prelude::*;

#[derive(Serialize, Deserialize)]
struct Resp {
    uri: String,
}

impl Api {
    pub fn get_connection_string(
        &self,
        id: String,
        branch_id: &str,
        database_name: &str,
        role_name: &str,
        pooled: bool,
    ) -> Result<String> {
        let req = ureq::get(&Endpoint::ConnectionString(id).to_string())
            .query("branch_id", branch_id)
            .query("database_name", database_name)
            .query("role_name", role_name)
            .query("pooled", &f!("{pooled}"));

        let resp: Resp = self.call(req)?;

        Ok(resp.uri)
    }
}
