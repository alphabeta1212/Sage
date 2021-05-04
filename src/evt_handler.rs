use serenity::model::channel::{
    Reaction,
    ReactionType::{Custom, Unicode},
};
use serenity::{model::gateway::Ready, prelude::*};

pub struct Handler;

impl EventHandler for Handler {
    // fn reaction_add(&self, ctx: Context, reaction: Reaction) {
    //     if let Err(why) = reaction.channel_id.say(
    //         &ctx.http,
    //         format!(
    //             "{} left a reaction {}",
    //             reaction.user(&ctx).unwrap().name,
    //             match reaction.emoji {
    //                 Custom {
    //                     animated: _,
    //                     id: _,
    //                     name,
    //                 } => name.unwrap(),
    //                 Unicode(uni) => uni,
    //                 _ => {
    //                     String::new()
    //                 }
    //             }
    //         ),
    //     ) {
    //         println!("Error reading reaction: {:?}", why);
    //     }
    // }

    // fn message(&self, _ctx: Context, _new_message: Message) {
    //     if _new_message.content == "!ping" {
    //         if let Err(failed) = _new_message.channel_id.say(&_ctx.http, "Pong!") {
    //             println!("Error sending message: {:?}", failed);
    //         }
    //     }
    // }

    fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }
}
