use serenity::framework::standard::{
    help_commands::with_embeds,
    macros::{command, group, help},
    Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::prelude::Context;
use std::collections::HashSet;

use crate::commands::api_calls::{get_genre_lists, get_top_books};

#[help]
#[command_not_found_text = "Command not found: `{}`"]
#[strikethrough_commands_tip_in_dm(" ")]
#[strikethrough_commands_tip_in_guild(" ")]
#[individual_command_tip = "!help <command> gives info about the specific command"]
#[lacking_permissions = "Nothing"]
#[lacking_role = "Nothing"]
#[lacking_ownership = "Strike"]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = with_embeds(context, msg, args, help_options, groups, owners);
    Ok(())
}

#[group]
#[prefix = "book"]
#[commands(ping, say, genres, bestof)]
struct General;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "Pong!").await?;
    Ok(())
}

#[command]
async fn genres(ctx: &Context, msg: &Message) -> CommandResult {
    let mut string = String::new();
    match get_genre_lists().await {
        Ok(list) => {
            for entry in list {
                string.push('\n');
                string.push_str(&entry);
            }
            msg.channel_id
                .say(
                    &ctx.http,
                    format!("Following genres are available:\n{}", string),
                )
                .await?;
        }
        Err(_) => {
            msg.channel_id
                .say(
                    &ctx.http,
                    format!("Sorry no Genres are available at this moment"),
                )
                .await?;
        }
    }
    Ok(())
}

#[command]
async fn bestof(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut string = String::new();
    match get_top_books(args.rest()).await {
        Ok(list) => {
            for (title, author, desc) in list {
                string.push_str(&format!(
                    "Book: {} \nAuthor: {} \nDescription: {}\n\n",
                    title, author, desc
                ));
            }
            msg.channel_id.say(&ctx.http, &string).await?;
        }
        Err(_) => {
            msg.channel_id
                .say(
                    &ctx.http,
                    &format!("No books found in {} genre", args.rest()),
                )
                .await?;
        }
    };
    Ok(())
}

#[command]
async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, args.rest()).await?;
    Ok(())
}
