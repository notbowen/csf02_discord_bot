extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod commands;

use std::env;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity_ctrlc::Ext;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            info!("Received command: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "start" => commands::minecraft::start::run(&command.data.options),
                _ => "Not Implemented!".to_string(),
            };

            if let Err(why) = command.create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(content))
            })
            .await

            {
                error!("Couldn't respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::minecraft::start::register(command))
        })
        .await;

        info!("Commands have been setup");
        debug!("Commands: {:#?}", commands);
    }
}

#[tokio::main]
async fn main() {
    // Init logging
    pretty_env_logger::init();

    // Get token
    let token = env::var("CSF02_TOKEN").expect("Find bot token");

    // Setup client
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Create client")
        .ctrlc()
        .expect("Hook CTRL C listener");

    // Start client
    if let Err(why) = client.start().await {
        error!("Unable to start client: {:?}", why);
    }
}
