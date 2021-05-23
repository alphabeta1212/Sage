use db_handler::Database;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use std::env;

#[group]
#[owners_only]
#[commands(everyone, set_quote_channel)]
#[only_in(guild)]
struct Admin;

#[command]
async fn everyone(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "@everyone").await?;
    Ok(())
}
#[command]
async fn set_quote_channel(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let allowed_types = [
        "inspire",
        "management",
        "sports",
        "life",
        "funny",
        "love",
        "art",
        "students",
    ];
    let mut valid = false;
    for types in allowed_types.iter() {
        if types == &args.rest() {
            let subscriber = msg.channel_id.0;
            let mut db = Database::new();
            let db_uri = env::var("MONGO_URI").unwrap();
            let _ = db.make_connection(db_uri).await;
            if let Err(_) = db.add_subscriber_to(types.to_string(), subscriber).await {
                println!("couldn't not update list");
            } else {
                println!("Updated List");
            }

            msg.channel_id
                .say(&ctx.http, "Channel subscribed for daily quotes")
                .await?;
            valid = true;
            break;
        }
    }
    if !valid {
        msg.channel_id
            .say(
                &ctx.http,
                format!(
                    "Please enter a valid quote Type. Following are available:\n_{:?}_",
                    allowed_types
                ),
            )
            .await?;
    }
    Ok(())
}
