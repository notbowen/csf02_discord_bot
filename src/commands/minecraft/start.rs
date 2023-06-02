// Starts the Minecraft server if it is not running
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use std::io::{BufRead, BufReader};
use std::fs::File;

use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn run(_options: &[CommandDataOption]) -> String {
    // Check if Minecraft server is already running
    let output = match Command::new("grep").arg("-q").arg("minecraft").output() {
        Ok(o) => o,
        Err(why) => {
            error!("Unable to execute pgrep: {}", why);
            return ":x: Error while checking for existing server".to_string();
        }
    };

    if !output.stdout.is_empty() {
        return ":x: Minecraft server is already running!".to_string();
    }

    // Start Minecraft server
    let builder = thread::Builder::new().name("minecraft".to_string());
    match builder.spawn(|| {
        if let Err(why) = start_server() {
            error!("Unable to start server: {}", why);
        }
    }) {
        Ok(_) => (),
        Err(_) => return ":x: An error occured while starting the server!".into(),
    }

    // Get IP address of server
    thread::sleep(Duration::from_secs(1));
    let server_ip = match get_server_ip("~/csf02_minecraft/server.log") {
        Ok(ip) => ip,
        Err(why) => {
            error!("Unable to get server IP: {}", why);
            return ":x: Server was started but bot was unable to get server's IP!".into();
        }
    };

    // Inform user of server IP
    format!(":white_check_mark: Successfully started server!\nServer Ip: `{}`", server_ip)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("start")
        .description("Starts the Minecraft server if it is not already running.")
}

fn get_server_ip(log_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(log_file)?);
    let mut lines = reader.lines();

    let ip_line = lines.nth(6).unwrap().ok().unwrap();
    let ip = ip_line.split(' ').last().unwrap().split("//").last().unwrap();

    Ok(ip.to_string())
}

fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("screen")
        .args([
            "-dmS",
            "minecraft",
            "bash",
            "-c",
            "~/csf02_minecraft/start_server.sh > ~/csf02_minecraft/server.log",
        ]) // INFO: Possible source of error
        .output()?;

    Ok(())
}
