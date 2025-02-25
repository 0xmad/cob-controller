use redis::Commands;
use serde::Deserialize;
use std::process::Command;
use sysinfo::{Pid, System};

use crate::config::get_env;

#[derive(Deserialize)]
pub struct CreateArgs {
    pub address: String,
}

#[derive(Deserialize)]
pub struct KillArgs {
    pub address: String,
    pub pid: u32,
}

pub fn create(connection: &mut redis::Connection, data: CreateArgs) -> Result<bool, &str> {
    let cmd = get_env("CHROMIUM_EXEC");
    let user_profiles_folder = get_env("USER_PROFILES_FOLDER");

    match Command::new(cmd)
        .arg(format!(
            "--user-data-dir={}/{}",
            user_profiles_folder, data.address
        ))
        .arg("--remote-debugging-port=9222")
        .spawn()
        .map(|output| {
            connection.set::<String, String, String>(data.address, output.id().to_string())
        }) {
        Ok(_) => Ok(true),
        Err(_) => Err("Failed to create process"),
    }
}

pub fn kill(connection: &mut redis::Connection, data: KillArgs) -> Result<bool, &str> {
    let system = System::new();

    system
        .process(Pid::from_u32(data.pid))
        .map(
            |process| match connection.del::<String, String>(data.address) {
                Ok(_) => process.kill(),
                Err(_) => false,
            },
        )
        .ok_or_else(|| "Failed to kill process")
}
