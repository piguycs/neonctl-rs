use inquire::Select;

use crate::prelude::*;

// 1. Select a project
// 2. Select a branch on that project
// 3. Use project id and branch id to get list of roles
// 4. Use project id and branch id to get list of databases
// 5. Get the connection uri
pub fn psql_command(api: Api) -> Result<()> {
    let projects = api.get_project_list()?;

    let select = Select::new("What project do you wanna connect to?", projects);

    let _ = select.prompt()?;

    todo!()
}
