// use crate::cmd_ctx_msg;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::channel::Message;
use serenity::prelude::Context;

#[group]
#[commands(bird, man, onion)]
#[description = "Sends emoji in the chat"]
#[default_command(bird)]
#[prefixes("em", "emoji")]
struct Emoji;

#[command]
#[description = "This man is weird"]
#[required_permissions("MANAGE_EMOJIS")]
async fn bird(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "🐦").await?;
    Ok(())
}

#[command]
#[description = "This man is weird"]
async fn man(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "🕴️").await?;
    Ok(())
}

#[command]
#[description = "For when you really wanna cry"]
async fn onion(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "🧅").await?;
    Ok(())
}
