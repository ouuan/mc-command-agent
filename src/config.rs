use anyhow::{Context, Result};
use serde::Deserialize;
use tokio::net::TcpStream;

#[derive(Deserialize)]
pub struct User {
    username: String,
    password: String,
    groups: Vec<String>,
}

#[derive(Deserialize)]
struct Group {
    name: String,
    commands: Vec<String>,
}

#[derive(Deserialize)]
pub struct Config {
    server_host: String,
    rcon_host: String,
    rcon_password: String,
    users: Vec<User>,
    groups: Vec<Group>,
}

impl Config {
    pub fn read() -> Result<Config> {
        let config_file = std::fs::read("config.yaml").context("Failed to read config file")?;
        serde_yaml::from_slice(&config_file).context("Failed to parse config file")
    }

    pub fn server_host(&self) -> String {
        self.server_host.clone()
    }

    pub async fn rcon_connection(&self) -> Result<rcon::Connection<TcpStream>> {
        rcon::Builder::new()
            .enable_minecraft_quirks(true)
            .connect(&self.rcon_host, &self.rcon_password)
            .await
            .context("Failed to connect to RCON server")
    }

    pub fn auth(&self, user_id: &str, password: &str) -> Option<&User> {
        self.users
            .iter()
            .find(|user| user.username == user_id && user.password == password)
    }

    pub fn commands_for_user(&self, user: &User) -> Vec<String> {
        self.groups
            .iter()
            .filter_map(|group| {
                if user.groups.contains(&group.name) {
                    Some(group.commands.iter())
                } else {
                    None
                }
            })
            .flatten()
            .cloned()
            .collect()
    }
}
