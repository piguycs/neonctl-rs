use serde::{Deserialize, Serialize};
use ureq::json;

use super::*;
use crate::{prelude::*, region::neon_regions};

#[derive(Debug, Serialize, Deserialize)]
struct Projects {
    pub projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatedProject {
    pub project: Project,
    pub connection_uris: Vec<ConnectionUris>,
}

// TODO: give it a meaningful name
#[derive(Debug, Serialize, Deserialize)]
struct RespProject {
    pub project: Project,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub region_id: String,
    pub name: String,
    pub created_at: String,
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionUris {
    pub connection_uri: String,
}

fn nearest_region() -> String {
    let mut regions: Vec<_> = neon_regions()
        .into_iter()
        .filter_map(|handle| handle.join().ok())
        .collect();

    regions.sort_by(|(_, time_a), (_, time_b)| time_a.cmp(time_b));

    regions
        .into_iter()
        .map(|(region, _)| region)
        .next()
        .expect("could not read any neon regions")
}

impl Api {
    pub fn get_project_list(&self) -> Result<Vec<Project>> {
        let res: Projects = self.call(ureq::get(&Endpoint::ProjectList.to_string()))?;
        Ok(res.projects)
    }

    // same schema as deleted project
    // TODO: Rename the struct
    pub fn get_project(&self, id: String) -> Result<Project> {
        let res: RespProject = self.call(ureq::get(&Endpoint::Project(id).to_string()))?;
        Ok(res.project)
    }

    pub fn get_project_by_name(&self, name: String) -> Result<Project> {
        for project in self.get_project_list()? {
            if project.name == name {
                return Ok(project);
            }
        }

        Err(f!("No project with name {name} found").into())
    }

    pub fn delete_project(&self, id: String) -> Result<Project> {
        let res: RespProject = self.call(ureq::delete(&Endpoint::ProjectDelete(id).to_string()))?;

        Ok(res.project)
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
