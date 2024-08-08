use clap::{Parser, Subcommand};
use prettytable::{format, row, Table};

use crate::{api::Api, prelude::*, table::print_table};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Me,
    Projects {
        #[command(subcommand)]
        opts: ProjectCommand,
    },
    Branches {
        #[command(subcommand)]
        opts: BranchCommand,
    },
    #[command(alias = "cs")]
    ConnectionString,
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
    },
    Delete {
        #[arg()]
        id: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum BranchCommand {}

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

                    println!("Projects");

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

                    print_table(row!["Id", "Name", "Region Id", "Created At"], data);

                    println!("Shared with me (TODO)");
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
                ProjectCommand::Get { id } => {
                    let res = api.get_project(id.to_owned())?;
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
                }
                ProjectCommand::Delete { id } => {
                    let res = api.delete_project(id.to_owned())?;
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
                }
            },
            Command::ConnectionString => {
                todo!("to be done")
            }
            Command::Branches { opts } => (),
        }

        Ok(())
    }
}
