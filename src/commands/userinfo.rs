use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::Args;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Colour;


#[command]
async fn get_info (ctx: &Context, msg: &Message, mut args: Args) -> CommandResult{
    let user_id = args.single::<u64>()?;
    let usr = ctx.http.get_user(user_id).await.expect("Invalid ID input");
    let member;

    member = ctx.http.get_member(*msg.guild_id.unwrap().as_u64(), usr.id.0).await;
    
    let creation_date = format!("<t:{0}:F> || <t:{0}:R>", usr.created_at().unix_timestamp());
    let joined_date;

    let nroles: String;
    let mut lroles = String::from("");

    match member {
        Ok(m) => {
            joined_date = format!("<t:{0}:F> || <t:{0}:R>", m.joined_at.unwrap().unix_timestamp());
            nroles = format!("N. Roles [{}]", m.roles.len());
            for role in m.roles {
                lroles.push_str(format!("<@&{}>, ", role.as_u64()).as_str());
            }
        },
        Err(_) => {
            joined_date = String::from("--- NA ---");
            nroles = String::from("N. Roles[0]");
            lroles = String::from("------------- NA -------------");
        },
    }

    if let Err(err) = msg.channel_id.send_message(&ctx.http, |m| {
        m.content("")
            .embed(|e| { e
                .author(|a| a
                    .name("Bot made by Hicster")
                    .url("https://github.com/HiccupEnthusiast/hiccup-pet")
                    .icon_url("https://cdn.discordapp.com/avatars/279692638083481602/eb220403280a99c50c61d729e8ac6e26.png?size=256"))
                .title(format!("{}#{}", usr.name, usr.discriminator))
                .description(format!("<@{}>", usr.id))
                .colour(Colour::from_rgb(53, 25, 7))
                .thumbnail(&usr.avatar_url().unwrap())
                .fields(vec![
                    ("Created: ", creation_date, true),
                    ("Joined: ", joined_date, true),
                ])
                .field(nroles,lroles, false)
                .footer(|f| f.text("footer").icon_url("https://files.catbox.moe/1ldihc.png"))
            })
    }).await {
        msg.reply(ctx, 
            format!("Error creating the message: {}", err)).await?;
    }

    Ok(())
}