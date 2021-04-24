use serenity::framework::standard::{
    help_commands,
    macros::{command, group, help},
    Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::prelude::Context;
use std::collections::HashSet;

#[help]
#[command_not_found_text = "Command not found: `{}`"]
#[strikethrough_commands_tip_in_dm(" ")]
#[strikethrough_commands_tip_in_guild(" ")]
#[individual_command_tip = "?help (command) gives info about the command"]
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
#[commands(ping, say)]
struct General;

#[command]

fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "Pong!")?;
    Ok(())
}

#[command]
fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(&ctx.http, args.rest())?;
    Ok(())
}
