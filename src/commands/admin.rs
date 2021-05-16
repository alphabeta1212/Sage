use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use std::fs::OpenOptions;
use std::io::Write;

#[group]
#[owners_only]
#[commands(everyone, set_quote_channel)]
#[only_in(guild)]
struct Admin;

#[command]
fn everyone(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "@everyone")?;
    Ok(())
}

#[command]
fn set_quote_channel(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let mut file = OpenOptions::new()
        .append(true)
        .open("sublist.txt")
        .expect("cannot open file");
    let subscriber = msg.channel_id.0;
    file.write_all(subscriber.to_string().as_bytes())
        .expect("failed to write channel ID");
    file.write_all("\n".as_bytes())
        .expect("failed to write new line");
    println!("{:?}", msg);
    msg.channel_id
        .say(&ctx.http, "Channel subscribed for daily quotes")?;
    Ok(())
}
