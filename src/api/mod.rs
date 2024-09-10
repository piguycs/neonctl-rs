mod branch;
mod connection_string;
mod project;
mod user;

use std::fmt::Display;

use inquire::{Password, PasswordDisplayMode};

use crate::prelude::*;

pub const NEON_ENDPOINT: &str = "https://console.neon.tech/api/v2";

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
    BranchCreate(String),

    /* GENERAL */
    ConnectionString(String),
}

impl Endpoint {
    pub fn get_base() -> String {
        NEON_ENDPOINT.to_string()
    }

    pub fn endpoint(&self) -> String {
        match self {
            Endpoint::Me => "/users/me".to_string(),
            Endpoint::ProjectList | Endpoint::ProjectCreate => "/projects".to_string(),
            Endpoint::ProjectDelete(id) | Endpoint::Project(id) => f!("/projects/{id}"),

            Endpoint::BranchList(id) | Endpoint::BranchCreate(id) => f!("/projects/{id}/branches"),

            Endpoint::ConnectionString(id) => f!("/projects/{id}/connection_uri"),
        }
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
