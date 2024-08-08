use std::{sync::Arc, thread, time};

use serde::{Deserialize, Serialize};
use ureq::json;

use super::*;
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Projects {
    pub projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatedProject {
    pub project: Project,
    pub connection_uris: Vec<ConnectionUris>,
}

// TODO: give it a meaningful name
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedProject {
    pub project: Project,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub region_id: String,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionUris {
    pub connection_uri: String,
}

fn nearest_region() -> String {
    let mut handles = vec![];

    for region in NEON_REGIONS {
        let region = Arc::new(region);
        let handle = thread::spawn(move || {
            let endpoint = f!("http://dynamodb.{region}.amazonaws.com");
            let start = time::Instant::now();
            ureq::get(&endpoint).call().unwrap();

            let time = start.elapsed();

            (f!("aws-{region}"), time.as_millis())
        });

        handles.push(handle);
    }

    let mut lowest_region = String::new();
    let mut lowest = -1;
    for handle in handles {
        let (region, time) = handle.join().unwrap();
        let time = time as i128;

        if lowest == -1 {
            lowest = time;
        } else if lowest > time {
            lowest = time;
            lowest_region = region;
        }
    }

    lowest_region
}

impl Api {
    pub fn get_project_list(&self) -> Result<Projects> {
        let res: Projects = self.call(ureq::get(&Endpoint::ProjectList.to_string()))?;
        Ok(res)
    }

    // same schema as deleted project
    // TODO: Rename the struct
    pub fn get_project(&self, id: String) -> Result<DeletedProject> {
        let res: DeletedProject = self.call(ureq::get(&Endpoint::Project(id).to_string()))?;
        Ok(res)
    }

    pub fn delete_project(&self, id: String) -> Result<DeletedProject> {
        let res: DeletedProject =
            self.call(ureq::delete(&Endpoint::ProjectDelete(id).to_string()))?;

        Ok(res)
    }

    pub fn create_project(
        &self,
        name: &Option<String>,
        region_id: &Option<String>,
    ) -> Result<CreatedProject> {
        let region = if let Some(region_id) = region_id {
            region_id.to_owned()
        } else {
            nearest_region()
        };

        let data = if let Some(name) = name {
            json!({
                "project": {
                    "pg_version": 16,
                    "region_id": region,
                    "name": name,
                }
            })
        } else {
            json!({
                "project": {
                    "pg_version": 16,
                    "region_id": region,
                }
            })
        };

        let res: CreatedProject = ureq::post(&Endpoint::ProjectCreate.to_string())
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .send_json(data)?
            .into_json()
            .expect("could not parse data into json");

        Ok(res)
    }
}

// TODO: test
// I would need to pay 20 euros if I wanna effectively test this :pepewhy:
