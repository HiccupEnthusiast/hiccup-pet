use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use serenity::utils::Colour;
use std::fmt;
use reqwest::Client;
use serde::{Serialize, Deserialize};
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize, Clone)]
struct Illustration {
    // Forces serde to parse id to id, seemingly increanses the chances 
    // of it catching the right field 
    #[serde(rename = "id")]
    id: u64,
    created_at: String,
    rating: String,
    tag_string_general: String,
    tag_string_character: String,
    tag_string_artist: String,
    tag_string_copyright: String,
    file_url: String,
    large_file_url: String,
}

impl Illustration {
    fn empty() -> Illustration {
        Illustration {
            id: 0,
            created_at: String::from(""),
            rating: String::from(""),
            tag_string_general: String::from(""),
            tag_string_character: String::from(""),
            tag_string_artist: String::from(""),
            tag_string_copyright: String::from(""),
            file_url: String::from(""),
            large_file_url: String::from(""),
        }

    }
}

impl fmt::Display for Illustration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Illustration #{} made by {} the {} which protray {} from {}.\n
                    Rated as `{}` and has the following tags: \n {} \n
                    url: {}\n large url: {}", 
                    self.id, self.tag_string_artist, self.created_at, self.tag_string_character,
                    self.tag_string_copyright, self.rating, self.tag_string_general,
                    self.file_url, self.large_file_url)
    }
}

#[command]
#[aliases("yuri", "y")]
async fn get_yuri(ctx: &Context, msg: &Message) -> CommandResult{
    let illust = match fetch_illustration(String::from("yuri")).await {
        Ok(ok) => ok,
        Err(_) => Illustration::empty(),
    };

    msg.channel_id.send_message(&ctx.http, |m| {
        m.content("")
            .embed(|e| { e
                .title(format!("Illustration made by: {}", illust.tag_string_artist))
                .description(illust.created_at)
                .colour(Colour::from_rgb(0, 254, 252))
                .fields(vec![
                    ("Character: ", illust.tag_string_character, true),
                    ("From: ", illust.tag_string_copyright, true),
                    ("Rating: ", illust.rating, true),
                ])
                .field("Tags: ",illust.tag_string_general, false)
                .image(illust.large_file_url)
                .footer(|f| f.text("").icon_url("https://files.catbox.moe/1ldihc.png"))
            })
        }).await?;
    Ok(())
}

async fn fetch_illustration(tags: String) -> Result<Illustration, Box<dyn std::error::Error>>{
    let rclient = Client::new();
    
    let resp: Vec<Illustration> = rclient.get
    ("https://safebooru.donmai.us/posts.json?random=true").json(&serde_json::json!({
        "tags": tags,
    })).send().await?.json().await?;

    let choosen_illust = resp.choose(&mut rand::thread_rng())
                                .expect("Couldn't choose a random illust");
    
    Ok(choosen_illust.clone())
}