use inquire::{Confirm, Select};

use crate::prelude::*;

pub fn get_connection_string(api: Api) -> Result<String> {
    let projects = api.get_project_list()?;
    let select = Select::new("What project do you wanna connect to?", projects);

    let project = select.prompt()?;

    let branches = api.get_branch_list(Some(project.id.clone()))?;
    let select = Select::new("What branch do you wanna connect to?", branches);

    let branch = select.prompt()?;

    let database = "neondb"; // TODO: dont hardcode lmao
    let role = "neondb_owner"; // TODO: dont hardcode lmao

    let pooled = Confirm::new("Do you want a pooled connection?").prompt()?;

    let cs = api.get_connection_string(project.id, &branch.id, database, role, pooled)?;

    Ok(cs)
}
