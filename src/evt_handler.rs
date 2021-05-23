use serenity::async_trait;
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

    // fn ready(&self, _ctx: Context, ready: Ready) {
    //     println!("{} is ready", ready.user.name);
    // }

    // async fn cache_ready(&self, _ctx: Context, _guilds: Vec<serenity::model::id::GuildId>) {}

    // async fn channel_create(
    //     &self,
    //     _ctx: Context,
    //     _channel: &serenity::model::channel::GuildChannel,
    // ) {
    // }

    // async fn category_create(
    //     &self,
    //     _ctx: Context,
    //     _category: &serenity::model::channel::ChannelCategory,
    // ) {
    // }

    // async fn category_delete(
    //     &self,
    //     _ctx: Context,
    //     _category: &serenity::model::channel::ChannelCategory,
    // ) {
    // }

    // async fn channel_delete(
    //     &self,
    //     _ctx: Context,
    //     _channel: &serenity::model::channel::GuildChannel,
    // ) {
    // }

    // async fn channel_pins_update(
    //     &self,
    //     _ctx: Context,
    //     _pin: serenity::model::event::ChannelPinsUpdateEvent,
    // ) {
    // }

    // async fn channel_update(
    //     &self,
    //     _ctx: Context,
    //     _old: Option<serenity::model::channel::Channel>,
    //     _new: serenity::model::channel::Channel,
    // ) {
    // }

    // async fn guild_ban_addition(
    //     &self,
    //     _ctx: Context,
    //     _guild_id: serenity::model::id::GuildId,
    //     _banned_user: serenity::model::prelude::User,
    // ) {
    // }

    // async fn guild_ban_removal(
    //     &self,
    //     _ctx: Context,
    //     _guild_id: serenity::model::id::GuildId,
    //     _unbanned_user: serenity::model::prelude::User,
    // ) {
    // }

    // async fn guild_create(
    //     &self,
    //     _ctx: Context,
    //     _guild: serenity::model::guild::Guild,
    //     _is_new: bool,
    // ) {
    // }

    // async fn guild_delete(
    //     &self,
    //     _ctx: Context,
    //     _incomplete: serenity::model::guild::GuildUnavailable,
    //     _full: Option<serenity::model::guild::Guild>,
    // ) {
    // }

    // async fn guild_emojis_update(
    //     &self,
    //     _ctx: Context,
    //     _guild_id: serenity::model::id::GuildId,
    //     _current_state: std::collections::HashMap<
    //         serenity::model::id::EmojiId,
    //         serenity::model::guild::Emoji,
    //     >,
    // ) {
    // }

    // async fn guild_integrations_update(
    //     &self,
    //     _ctx: Context,
    //     _guild_id: serenity::model::id::GuildId,
    // ) {
    // }

    // async fn guild_member_addition(
    //     &self,
    //     _ctx: Context,
    //     _guild_id: serenity::model::id::GuildId,
    //     _new_member: serenity::model::guild::Member,
    // ) {
    // }

    // async fn guild_member_removal(
    //     &self,
    //     _ctx: Context,
    //     _guild_id: serenity::model::id::GuildId,
    //     _user: serenity::model::prelude::User,
    //     _member_data_if_available: Option<serenity::model::guild::Member>,
    // ) {
    // }

    // async fn guild_member_update(
    //     &self,
    //     _ctx: Context,
    //     _old_if_available: Option<serenity::model::guild::Member>,
    //     _new: serenity::model::guild::Member,
    // ) {
    // }

    // async fn guild_members_chunk(
    //     &self,
    //     _ctx: Context,
    //     _chunk: serenity::model::event::GuildMembersChunkEvent,
    // ) {
    // }

    // async fn guild_role_create(
    //     &self,
    //     _ctx: Context,
    //     _guild_id: serenity::model::id::GuildId,
    //     _new: serenity::model::guild::Role,
    // ) {
    // }

    // async fn guild_role_delete(
    //     &self,
    //     _ctx: Context,
    //     _guild_id: serenity::model::id::GuildId,
    //     _removed_role_id: serenity::model::id::RoleId,
    //     _removed_role_data_if_available: Option<serenity::model::guild::Role>,
    // ) {
    // }

    // async fn guild_role_update(
    //     &self,
    //     _ctx: Context,
    //     _guild_id: serenity::model::id::GuildId,
    //     _old_data_if_available: Option<serenity::model::guild::Role>,
    //     _new: serenity::model::guild::Role,
    // ) {
    // }

    // async fn guild_unavailable(&self, _ctx: Context, _guild_id: serenity::model::id::GuildId) {}

    // async fn guild_update(
    //     &self,
    //     _ctx: Context,
    //     _old_data_if_available: Option<serenity::model::guild::Guild>,
    //     _new_but_incomplete: serenity::model::guild::PartialGuild,
    // ) {
    // }

    // async fn invite_create(&self, _ctx: Context, _data: serenity::model::event::InviteCreateEvent) {
    // }

    // async fn invite_delete(&self, _ctx: Context, _data: serenity::model::event::InviteDeleteEvent) {
    // }

    // async fn message(&self, _ctx: Context, _new_message: serenity::model::channel::Message) {}

    // async fn message_delete(
    //     &self,
    //     _ctx: Context,
    //     _channel_id: serenity::model::id::ChannelId,
    //     _deleted_message_id: serenity::model::id::MessageId,
    //     _guild_id: Option<serenity::model::id::GuildId>,
    // ) {
    // }

    // async fn message_delete_bulk(
    //     &self,
    //     _ctx: Context,
    //     _channel_id: serenity::model::id::ChannelId,
    //     _multiple_deleted_messages_ids: Vec<serenity::model::id::MessageId>,
    //     _guild_id: Option<serenity::model::id::GuildId>,
    // ) {
    // }

    // async fn message_update(
    //     &self,
    //     _ctx: Context,
    //     _old_if_available: Option<serenity::model::channel::Message>,
    //     _new: Option<serenity::model::channel::Message>,
    //     _event: serenity::model::event::MessageUpdateEvent,
    // ) {
    // }

    // async fn reaction_add(&self, _ctx: Context, _add_reaction: serenity::model::channel::Reaction) {
    // }

    // async fn reaction_remove(
    //     &self,
    //     _ctx: Context,
    //     _removed_reaction: serenity::model::channel::Reaction,
    // ) {
    // }

    // async fn reaction_remove_all(
    //     &self,
    //     _ctx: Context,
    //     _channel_id: serenity::model::id::ChannelId,
    //     _removed_from_message_id: serenity::model::id::MessageId,
    // ) {
    // }

    // async fn presence_replace(&self, _ctx: Context, _: Vec<serenity::model::prelude::Presence>) {}

    // async fn presence_update(
    //     &self,
    //     _ctx: Context,
    //     _new_data: serenity::model::event::PresenceUpdateEvent,
    // ) {
    // }

    // async fn resume(&self, _ctx: Context, _: serenity::model::event::ResumedEvent) {}

    // async fn shard_stage_update(
    //     &self,
    //     _ctx: Context,
    //     _: serenity::client::bridge::gateway::event::ShardStageUpdateEvent,
    // ) {
    // }

    // async fn typing_start(&self, _ctx: Context, _: serenity::model::event::TypingStartEvent) {}

    // async fn unknown(&self, _ctx: Context, _name: String, _raw: serenity::model::prelude::Value) {}

    // async fn user_update(
    //     &self,
    //     _ctx: Context,
    //     _old_data: serenity::model::prelude::CurrentUser,
    //     _new: serenity::model::prelude::CurrentUser,
    // ) {
    // }

    // async fn voice_server_update(
    //     &self,
    //     _ctx: Context,
    //     _: serenity::model::event::VoiceServerUpdateEvent,
    // ) {
    // }

    // async fn voice_state_update(
    //     &self,
    //     _ctx: Context,
    //     _: Option<serenity::model::id::GuildId>,
    //     _old: Option<serenity::model::prelude::VoiceState>,
    //     _new: serenity::model::prelude::VoiceState,
    // ) {
    // }

    // async fn webhook_update(
    //     &self,
    //     _ctx: Context,
    //     _guild_id: serenity::model::id::GuildId,
    //     _belongs_to_channel_id: serenity::model::id::ChannelId,
    // ) {
    // }

    // async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {}
}
