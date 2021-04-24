use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::channel::Message;
use serenity::prelude::Context;

#[group]
#[owners_only]
#[commands(everyone)]
#[only_in(guild)]
struct Admin;

#[command]
fn everyone(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "@everyone")?;
    Ok(())
}
