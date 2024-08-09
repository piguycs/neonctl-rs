mod branch;
mod project;
mod user;

use std::fmt::Display;

use inquire::{Password, PasswordDisplayMode};

use crate::prelude::*;

fn get_api_key() -> Result<String> {
    let entry = keyring::Entry::new("neonctl-rs", "api")?;

    let key = match entry.get_password() {
        Ok(key) => key,
        Err(keyring::Error::NoEntry) => {
            println!("Generate a new key at https://console.neon.tech/app/settings/api-keys");
            let pask = Password::new("Enter API key (CTRL+R to reveal):")
                .without_confirmation()
                .with_display_toggle_enabled()
                .with_display_mode(PasswordDisplayMode::Masked);

            let key = pask.prompt()?;

            entry.set_password(&key)?;

            key
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };

    Ok(key)
}

pub struct Api {
    api_key: String,
}

pub enum Endpoint {
    /* USER */
    Me,
    /* PROJECT */
    ProjectList,
    Project(String),
    ProjectCreate,
    ProjectDelete(String),
    /* BRANCH */
    BranchList(String),
    #[allow(unused)]
    BranchCreate(String),
}

impl Endpoint {
    pub fn get_base() -> String {
        "https://console.neon.tech/api/v2".to_string()
    }

    pub fn endpoint(&self) -> String {
        let endpoint = match self {
            Endpoint::Me => "/users/me",
            Endpoint::ProjectList | Endpoint::ProjectCreate => "/projects",
            Endpoint::ProjectDelete(id) | Endpoint::Project(id) => &f!("/projects/{id}"),

            Endpoint::BranchList(id) | Endpoint::BranchCreate(id) => &f!("/projects/{id}/branches"),
        };

        endpoint.to_string()
    }
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", Self::get_base(), self.endpoint())
    }
}

impl Api {
    pub fn try_new() -> Result<Self> {
        Ok(Self {
            api_key: get_api_key()?,
        })
    }

    pub fn call<T: serde::de::DeserializeOwned>(&self, req: ureq::Request) -> Result<T> {
        let json: T = req
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .call()?
            .into_json()?;

        Ok(json)
    }
}
