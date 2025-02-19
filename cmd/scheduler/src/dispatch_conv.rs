use dbrokerapi::broker_scheduler_rpc::{DiscordEvent, DiscordEventData};
use runtime_models::internal::events::VoiceState;
use twilight_model::id::{marker::GuildMarker, Id};

pub fn discord_event_to_dispatch(evt: DiscordEvent) -> Option<DiscordDispatchEvent> {
    match evt.event {
        DiscordEventData::MessageCreate(m) => Some(DiscordDispatchEvent {
            name: "MESSAGE_CREATE",
            guild_id: evt.guild_id,
            data: serde_json::to_value(&runtime_models::internal::messages::Message::from(m.0))
                .unwrap(),
        }),
        DiscordEventData::MessageUpdate(m) => Some(DiscordDispatchEvent {
            name: "MESSAGE_UPDATE",
            guild_id: evt.guild_id,
            data: serde_json::to_value(runtime_models::internal::events::EventMessageUpdate::from(
                *m,
            ))
            .unwrap(),
        }),
        DiscordEventData::MessageDelete(m) => Some(DiscordDispatchEvent {
            name: "MESSAGE_DELETE",
            guild_id: evt.guild_id,
            data: serde_json::to_value(runtime_models::discord::events::EventMessageDelete::from(
                m,
            ))
            .unwrap(),
        }),
        DiscordEventData::MemberAdd(m) => Some(DiscordDispatchEvent {
            name: "MEMBER_ADD",
            guild_id: m.guild_id,
            data: serde_json::to_value(runtime_models::internal::member::Member::from(m.member))
                .unwrap(),
        }),
        DiscordEventData::MemberUpdate(m) => Some(DiscordDispatchEvent {
            name: "MEMBER_UPDATE",
            guild_id: m.guild_id,
            data: serde_json::to_value(runtime_models::internal::member::Member::from(*m)).unwrap(),
        }),
        DiscordEventData::MemberRemove(m) => Some(DiscordDispatchEvent {
            name: "MEMBER_REMOVE",
            guild_id: m.guild_id,
            data: serde_json::to_value(runtime_models::internal::events::EventMemberRemove::from(
                m,
            ))
            .unwrap(),
        }),
        DiscordEventData::ReactionAdd(r) => Some(DiscordDispatchEvent {
            name: "MESSAGE_REACTION_ADD",
            guild_id: r.guild_id.expect("only guild event sent to guild worker"),
            data: serde_json::to_value(
                runtime_models::internal::events::EventMessageReactionAdd::from(*r),
            )
            .unwrap(),
        }),
        DiscordEventData::ReactionRemove(r) => Some(DiscordDispatchEvent {
            name: "MESSAGE_REACTION_REMOVE",
            guild_id: r.guild_id.expect("only guild event sent to guild worker"),
            data: serde_json::to_value(
                runtime_models::discord::events::EventMessageReactionRemove::from(*r),
            )
            .unwrap(),
        }),
        DiscordEventData::ReactionRemoveAll(r) => Some(DiscordDispatchEvent {
            name: "MESSAGE_REACTION_REMOVE_ALL",
            guild_id: r.guild_id.expect("only guild event sent to guild worker"),
            data: serde_json::to_value(
                runtime_models::discord::events::EventMessageReactionRemoveAll::from(r),
            )
            .unwrap(),
        }),
        DiscordEventData::ReactionRemoveEmoji(r) => Some(DiscordDispatchEvent {
            name: "MESSAGE_REACTION_REMOVE_ALL_EMOJI",
            guild_id: r.guild_id,
            data: serde_json::to_value(
                runtime_models::discord::events::EventMessageReactionRemoveAllEmoji::from(r),
            )
            .unwrap(),
        }),
        DiscordEventData::ChannelCreate(cc) => Some(DiscordDispatchEvent {
            name: "CHANNEL_CREATE",
            guild_id: evt.guild_id,
            data: serde_json::to_value(&runtime_models::internal::channel::GuildChannel::from(
                cc.0,
            ))
            .unwrap(),
        }),
        DiscordEventData::ChannelUpdate(cu) => Some(DiscordDispatchEvent {
            name: "CHANNEL_UPDATE",
            guild_id: evt.guild_id,
            data: serde_json::to_value(&runtime_models::internal::channel::GuildChannel::from(
                cu.0,
            ))
            .unwrap(),
        }),
        DiscordEventData::ChannelDelete(cd) => Some(DiscordDispatchEvent {
            name: "CHANNEL_DELETE",
            guild_id: evt.guild_id,
            data: serde_json::to_value(&runtime_models::internal::channel::GuildChannel::from(
                cd.0,
            ))
            .unwrap(),
        }),
        DiscordEventData::ThreadCreate(r) => Some(DiscordDispatchEvent {
            name: "THREAD_CREATE",
            guild_id: evt.guild_id,
            data: serde_json::to_value(&runtime_models::internal::channel::GuildChannel::from(r.0))
                .unwrap(),
        }),
        DiscordEventData::ThreadUpdate(r) => Some(DiscordDispatchEvent {
            name: "THREAD_UPDATE",
            guild_id: evt.guild_id,
            data: serde_json::to_value(&runtime_models::internal::channel::GuildChannel::from(r.0))
                .unwrap(),
        }),
        DiscordEventData::ThreadDelete(r) => Some(DiscordDispatchEvent {
            name: "THREAD_DELETE",
            guild_id: evt.guild_id,
            data: serde_json::to_value(runtime_models::discord::events::EventThreadDelete::from(r))
                .unwrap(),
        }),
        DiscordEventData::ThreadListSync(r) => Some(DiscordDispatchEvent {
            name: "THREAD_LIST_SYNC",
            guild_id: evt.guild_id,
            data: serde_json::to_value(
                runtime_models::internal::events::EventThreadListSync::from(r),
            )
            .unwrap(),
        }),
        DiscordEventData::ThreadMemberUpdate(r) => Some(DiscordDispatchEvent {
            name: "THREAD_MEMBER_UPDATE",
            guild_id: evt.guild_id,
            data: serde_json::to_value(runtime_models::internal::channel::ThreadMember::from(
                r.member,
            ))
            .unwrap(),
        }),
        DiscordEventData::ThreadMembersUpdate(r) => Some(DiscordDispatchEvent {
            name: "THREAD_MEMBERS_UPDATE",
            guild_id: r.guild_id,
            data: serde_json::to_value(
                runtime_models::internal::events::EventThreadMembersUpdate::from(r),
            )
            .unwrap(),
        }),
        DiscordEventData::InteractionCreate(interaction) => {
            let guild_id = evt.guild_id;

            if let Ok(v) =
                runtime_models::internal::interaction::Interaction::try_from(interaction.0)
            {
                match v {
                    runtime_models::internal::interaction::Interaction::Command(
                        cmd_interaction,
                    ) => Some(DiscordDispatchEvent {
                        guild_id,
                        name: "BOTLOADER_COMMAND_INTERACTION_CREATE",
                        data: serde_json::to_value(&cmd_interaction).unwrap(),
                    }),
                    runtime_models::internal::interaction::Interaction::MessageComponent(
                        component_interaction,
                    ) => Some(DiscordDispatchEvent {
                        guild_id,
                        name: "BOTLOADER_COMPONENT_INTERACTION_CREATE",
                        data: serde_json::to_value(&component_interaction).unwrap(),
                    }),
                    runtime_models::internal::interaction::Interaction::ModalSubmit(
                        modal_interaction,
                    ) => Some(DiscordDispatchEvent {
                        guild_id,
                        name: "BOTLOADER_MODAL_SUBMIT_INTERACTION_CREATE",
                        data: serde_json::to_value(&modal_interaction).unwrap(),
                    }),
                }
            } else {
                None
            }
        }
        DiscordEventData::InviteCreate(invite) => Some(DiscordDispatchEvent {
            guild_id: invite.guild_id,
            name: "INVITE_CREATE",
            data: serde_json::to_value(
                runtime_models::internal::events::EventInviteCreate::try_from(*invite)
                    .map_err(|err| {
                        tracing::error!(
                            "failed converting dispatch event InviteCreate event: {err:?}"
                        );
                    })
                    .ok(),
            )
            .unwrap(),
        }),
        DiscordEventData::InviteDelete(invite) => Some(DiscordDispatchEvent {
            guild_id: invite.guild_id,
            name: "INVITE_DELETE",
            data: serde_json::to_value(runtime_models::internal::events::EventInviteDelete::from(
                invite,
            ))
            .unwrap(),
        }),
        DiscordEventData::VoiceStateUpdate { event, old_state } => Some(DiscordDispatchEvent {
            guild_id: evt.guild_id,
            name: "VOICE_STATE_UPDATE",
            data: serde_json::to_value(runtime_models::internal::events::EventVoiceStateUpdate {
                new: VoiceState::try_from(event.0).ok()?,
                old: old_state
                    .map(|v| VoiceState::try_from(*v))
                    .transpose()
                    .ok()?,
            })
            .unwrap(),
        }),
        DiscordEventData::GuildDelete(_) => None,
        DiscordEventData::GuildCreate(_) => None,
        DiscordEventData::MessageDeleteBulk(_) => None,
    }
}

pub struct DiscordDispatchEvent {
    pub guild_id: Id<GuildMarker>,
    pub name: &'static str,
    pub data: serde_json::Value,
}
