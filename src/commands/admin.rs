use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn check(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx,
        format!("Oh hello <@{}>! I'm alive, thanks for checking n.n", msg.author.id)).await?;
    return Ok(())
}
