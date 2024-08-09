use clap::{Parser, Subcommand};
use prettytable::row;

use crate::{prelude::*, region::neon_regions};

#[derive(Parser, Debug)]
#[command(styles = CLAP_STYLING)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Me,
    #[command(alias = "project")]
    Projects {
        #[command(subcommand)]
        opts: ProjectCommand,
    },
    #[command(alias = "branch")]
    Branches {
        #[command(subcommand)]
        opts: BranchCommand,
    },
    #[command(alias = "cs")]
    ConnectionString,

    /// Display neondb regions and their ping from your machine
    #[command(alias = "region")]
    Regions,
}

#[derive(Subcommand, Debug)]
pub enum ProjectCommand {
    List,
    Create {
        #[arg(long, short)]
        name: Option<String>,
        #[arg(long, short)]
        region_id: Option<String>,
    },
    Get {
        #[arg()]
        id: String,
        #[arg(long, short)]
        name: bool,
    },
    Delete {
        #[arg()]
        id: String,
        #[arg(long, short)]
        name: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum BranchCommand {
    List {
        #[arg()]
        id: Option<String>,
    },
}

impl Command {
    pub fn run(&self, api: Api) -> Result<()> {
        match self {
            Command::Me => {
                let data = api.get_user_details()?;

                print_table(
                    row!["Login", "Email", "Name", "Projects Limit"],
                    vec![row![data.login, data.email, data.name, data.projects_limit]],
                );
            }
            Command::Projects { opts } => match opts {
                ProjectCommand::List => {
                    let res = api.get_project_list()?;

                    print_bold("Projects");

                    let data: Vec<_> = res
                        .projects
                        .iter()
                        .map(|project| {
                            row![
                                project.id,
                                project.name,
                                project.region_id,
                                project.created_at
                            ]
                        })
                        .collect();

                    if !data.is_empty() {
                        print_table(row!["Id", "Name", "Region Id", "Created At"], data);
                    } else {
                        println!("No projects found on your account\n");
                    }

                    print_bold("Shared with me (TODO)");
                }
                ProjectCommand::Create { name, region_id } => {
                    let res = api
                        .create_project(name, region_id)
                        .expect("bad request to api");
                    let project = res.project;
                    print_table(
                        row!["Id", "Name", "Region Id", "Created At"],
                        vec![row![
                            project.id,
                            project.name,
                            project.region_id,
                            project.created_at
                        ]],
                    );

                    let data: Vec<_> = res
                        .connection_uris
                        .iter()
                        .map(|uri| row![uri.connection_uri])
                        .collect();

                    print_table(row!["Connection URI"], data);
                }
                ProjectCommand::Get { id, name } => {
                    let project = if *name {
                        api.get_project_by_name(id.to_owned())?
                    } else {
                        api.get_project(id.to_owned())?
                    };

                    print_table(
                        row!["Id", "Name", "Region Id", "Created At"],
                        vec![row![
                            project.id,
                            project.name,
                            project.region_id,
                            project.created_at
                        ]],
                    );
                }
                ProjectCommand::Delete { id, name } => {
                    let id = if *name {
                        let project = api.get_project_by_name(id.to_owned())?;
                        project.id
                    } else {
                        id.to_owned()
                    };

                    let project = api.delete_project(id)?;

                    print_table(
                        row!["Id", "Name", "Region Id", "Created At"],
                        vec![row![
                            project.id,
                            project.name,
                            project.region_id,
                            project.created_at
                        ]],
                    );
                }
            },
            Command::ConnectionString => {
                todo!("to be done")
            }
            Command::Branches { opts } => match opts {
                BranchCommand::List { id } => {
                    let branches = api.get_branch_list(id.to_owned())?;

                    let data: Vec<_> = branches
                        .iter()
                        .map(|branch| {
                            row![
                                branch.id,
                                branch.name,
                                branch.primary,
                                branch.default,
                                branch.created_at,
                                branch.updated_at
                            ]
                        })
                        .collect();

                    print_table(
                        row![
                            "Id",
                            "Name",
                            "Primary",
                            "Default",
                            "Created At",
                            "Updated At"
                        ],
                        data,
                    )
                }
            },
            Command::Regions => {
                let mut regions: Vec<_> = neon_regions()
                    .into_iter()
                    .filter_map(|handle| handle.join().ok())
                    .collect();

                regions.sort_by(|(_, time_a), (_, time_b)| time_a.cmp(time_b));

                let data: Vec<_> = regions
                    .iter()
                    .map(|(region, time)| row![region, f!("{time}ms")])
                    .collect();

                print_table(row!["Region", "Ping"], data);
            }
        }

        Ok(())
    }
}
