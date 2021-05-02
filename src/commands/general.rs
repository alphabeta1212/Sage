use serenity::framework::standard::{
    help_commands,
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
fn help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}

#[group]
#[commands(ping, say, genres, bestof)]
struct General;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "Pong!")?;
    Ok(())
}

#[command]
fn genres(ctx: &mut Context, msg: &Message) -> CommandResult {
    let list = get_genre_lists();
    let mut string = String::new();
    for entry in list.unwrap() {
        string.push('\n');
        string.push_str(&entry);
    }
    msg.reply(&ctx, format!("Following genres are available:\n{}", string))?;
    Ok(())
}

#[command]
fn bestof(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let mut string = String::new();
    match get_top_books(args.rest()) {
        Ok(list) => {
            for (title, author, desc) in list {
                string.push_str(&format!(
                    "Book: {} \nAuthor: {} \nDescription: {}\n\n",
                    title, author, desc
                ));
            }
            msg.channel_id.say(&ctx.http, &string)?;
        }
        Err(_) => {
            msg.channel_id.say(
                &ctx.http,
                &format!("No books found in {} genre", args.rest()),
            )?;
        }
    };
    Ok(())
}

#[command]
fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, args.rest())?;
    Ok(())
}
