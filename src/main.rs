mod commands;

use std::{collections::{HashSet, HashMap}, env, sync::Arc};
use tokio::sync::Mutex;
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, standard::StandardFramework},
    model::{gateway::{GatewayIntents, Ready}, id::UserId},
    prelude::*,
};

use crate::commands::admin::*;
use crate::commands::userinfo::*;
use crate::commands::imagen::*;

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct CommandCounter;
impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

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

#[group]
#[commands(get_info, get_yuri)]
struct General;

#[group]
#[summary = "Various admin/info commands meant only for staff eyes"]
#[commands(check)]
struct Admin;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env");
    let token = env::var("DTOKEN").expect("Expected token");
    let owners_ids = env::var("ADMINS").expect("Expected CSV");

    let mut owners = HashSet::new() as HashSet<UserId>;
    for n in owners_ids.split(","){
        owners.insert(UserId(n.parse().unwrap()));
    }

    let framework = StandardFramework::new()
        .configure(|c| c
                   .with_whitespace(true)
                   .prefix("h:")
                   .delimiters(vec![", ", ","])
                   .owners(owners))
        .group(&GENERAL_GROUP)
        .group(&ADMIN_GROUP);

    let intents = GatewayIntents::all();
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .type_map_insert::<CommandCounter>(HashMap::default())
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    if let Err(err) = client.start().await {
        println!("Error starting the client: {:?}", err);
    }
}
