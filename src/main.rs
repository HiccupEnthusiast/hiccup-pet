use std::{collections::HashSet, env};

use serenity::{
    async_trait,
    framework::{standard::macros::group, StandardFramework},
    model::{gateway::Ready, id::UserId},
    prelude::*,
};

#[group]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, bot: Ready) {
        println!(
            "{}#{} succesfully connected!",
            bot.user.name, bot.user.discriminator
        );
    }
}
#[tokio::main]
async fn main() {
    // .env must be in ROOT dir, not in /src
    dotenv::dotenv().expect("Failed to load .env");
    let token = env::var("DTOKEN").expect("Expected token");
    let owners_ids = env::var("ADMINS").expect("Expected CSV");

    let mut owners = HashSet::new() as HashSet<UserId>;
    for n in owners_ids.split(",") {
        owners.insert(UserId(n.parse().unwrap()));
    }

    let intents = GatewayIntents::GUILDS 
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_TYPING
        | GatewayIntents::DIRECT_MESSAGES;

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix(":h"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    

    if let Err(err) = client.start().await {
        panic!("Error starting the client: {:?}", err);
    };
}
