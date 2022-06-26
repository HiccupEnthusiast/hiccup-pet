use std::{env, collections::HashSet};

use serenity::{
    async_trait,
    model::{gateway::Ready, id::UserId},
    prelude::*,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
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
    dotenv::dotenv().expect("Failed to load .env");
    let token = env::var("DTOKEN").expect("Expected token");
    let owners_ids = env::var("ADMINS")
        .expect("Expected CSV");
    
    let mut owners = HashSet::new() as HashSet<UserId>;
    for n in owners_ids.split(",") {
            owners.insert(UserId(n.parse().unwrap()));
    }

    let framework = 
        StandardFramework::new().configure
            (|c| c
                .owners(owners)
                .prefix(":h"))
            .group(&GENERAL_GROUP);

    let bot = Http::new_with_token(&token);
    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    
    println!("d");
    if let Err(err) = client.start().await {
        panic!("Error starting the client: {:?}", err);
    };
}
