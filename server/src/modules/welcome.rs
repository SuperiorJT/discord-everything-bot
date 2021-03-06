use std::error::Error;

use twilight_model::{
    channel::Channel,
    gateway::payload::{MemberAdd, MemberRemove},
    guild::Guild,
    id::{ChannelId, RoleId},
    user::User,
};

use crate::{
    bot::event_handler::EventHandler,
    db::queries::welcome::WelcomeModule,
    models::embed::Embed,
    util::cdn::{GuildIconUrl, SupportsPng, UserAvatarUrl},
};

#[derive(sqlx::FromRow, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeExpanded {
    pub id: i64,
    pub guild_id: String,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join: Option<WelcomeJoinContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_dm: Option<WelcomeJoinDmContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_roles: Option<WelcomeJoinRolesContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave: Option<WelcomeLeaveContent>,
}

impl WelcomeExpanded {
    pub fn from_module(m: WelcomeModule) -> Self {
        WelcomeExpanded {
            id: m.id,
            guild_id: m.guild_id,
            enabled: m.enabled,
            join: None,
            join_dm: None,
            join_roles: None,
            leave: None,
        }
    }
}

impl From<WelcomeExpandedRow> for WelcomeExpanded {
    fn from(w: WelcomeExpandedRow) -> Self {
        let mut val = WelcomeExpanded {
            id: w.id,
            guild_id: w.guild_id,
            enabled: w.enabled,
            join: None,
            join_dm: None,
            join_roles: None,
            leave: None,
        };
        if w.join_enabled.is_some() {
            val.join = Some(WelcomeJoinContent {
                enabled: w.join_enabled,
                message_type: w.join_message_type,
                channel_id: w.join_channel_id,
                content: w.join_content,
                embed: w.join_embed.and_then(|val| serde_json::from_str(&val).ok()),
            });
        }
        if w.join_dm_enabled.is_some() {
            val.join_dm = Some(WelcomeJoinDmContent {
                enabled: w.join_dm_enabled,
                message_type: w.join_dm_message_type,
                content: w.join_dm_content,
                embed: w
                    .join_dm_embed
                    .and_then(|val| serde_json::from_str(&val).ok()),
            });
        }
        if w.join_roles_enabled.is_some() {
            val.join_roles = Some(WelcomeJoinRolesContent {
                enabled: w.join_roles_enabled,
                roles: w
                    .join_roles_roles
                    .and_then(|val| serde_json::from_str(&val).ok()),
                delay: w.join_roles_delay,
            });
        }
        if w.leave_enabled.is_some() {
            val.leave = Some(WelcomeLeaveContent {
                enabled: w.leave_enabled,
                channel_id: w.leave_channel_id,
                content: w.leave_content,
            });
        }
        val
    }
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct WelcomeExpandedRow {
    pub id: i64,
    pub guild_id: String,
    pub enabled: bool,
    pub join_enabled: Option<bool>,
    pub join_message_type: Option<String>,
    pub join_channel_id: Option<String>,
    pub join_content: Option<String>,
    pub join_embed: Option<String>,
    pub join_dm_enabled: Option<bool>,
    pub join_dm_message_type: Option<String>,
    pub join_dm_content: Option<String>,
    pub join_dm_embed: Option<String>,
    pub join_roles_enabled: Option<bool>,
    pub join_roles_roles: Option<String>,
    pub join_roles_delay: Option<bool>,
    pub leave_enabled: Option<bool>,
    pub leave_channel_id: Option<String>,
    pub leave_content: Option<String>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeContent {
    pub enabled: Option<bool>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeJoin {
    pub id: i64,
    pub enabled: bool,
    pub message_type: String,
    pub channel_id: String,
    pub content: String,
    pub embed: serde_json::Value,
}

#[derive(sqlx::FromRow, serde::Serialize, sqlx::Type, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeJoinContent {
    pub enabled: Option<bool>,
    pub message_type: Option<String>,
    pub channel_id: Option<String>,
    pub content: Option<String>,
    pub embed: Option<serde_json::Value>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeJoinDm {
    pub id: i64,
    pub enabled: bool,
    pub message_type: String,
    pub content: String,
    pub embed: serde_json::Value,
}

#[derive(sqlx::FromRow, serde::Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeJoinDmContent {
    pub enabled: Option<bool>,
    pub message_type: Option<String>,
    pub content: Option<String>,
    pub embed: Option<serde_json::Value>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeJoinRoles {
    pub id: i64,
    pub enabled: bool,
    pub roles: serde_json::Value,
    pub delay: bool,
}

#[derive(sqlx::FromRow, serde::Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeJoinRolesContent {
    pub enabled: Option<bool>,
    pub roles: Option<serde_json::Value>,
    pub delay: Option<bool>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeLeave {
    pub id: i64,
    pub enabled: bool,
    pub channel_id: String,
    pub content: String,
}
#[derive(sqlx::FromRow, serde::Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeLeaveContent {
    pub enabled: Option<bool>,
    pub channel_id: Option<String>,
    pub content: Option<String>,
}

pub async fn handle_member_add(
    member_add: Box<MemberAdd>,
    event_handler: &EventHandler<'_>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let guild = event_handler
        .bot
        .http
        .guild(member_add.guild_id)
        .exec()
        .await?
        .model()
        .await?;

    let join_config = event_handler
        .bot
        .db
        .welcome()
        .module_join_fetch_by_guild_id(guild.id.0)
        .await?;

    if join_config.enabled {
        match join_config.message_type.as_str() {
            "text" => {
                member_join_content(join_config, &guild, &member_add.user, event_handler).await?;
            }
            "embed" => {
                member_join_embed(join_config, &guild, &member_add.user, event_handler).await?;
            }
            _ => {
                return Err("Invalid mesage type on welcome_join".into());
            }
        }
    }

    let join_dm_config = event_handler
        .bot
        .db
        .welcome()
        .module_join_dm_fetch_by_guild_id(guild.id.0)
        .await?;

    if join_dm_config.enabled {
        match join_dm_config.message_type.as_str() {
            "text" => {
                member_join_dm_content(join_dm_config, &guild, &member_add.user, event_handler)
                    .await?;
            }
            "embed" => {
                member_join_dm_embed(join_dm_config, &guild, &member_add.user, event_handler)
                    .await?;
            }
            _ => {
                return Err("Invalid mesage type on welcome_join".into());
            }
        }
    }

    let join_roles_config = event_handler
        .bot
        .db
        .welcome()
        .module_join_roles_fetch_by_guild_id(guild.id.0)
        .await?;

    if join_roles_config.enabled {
        let roles = serde_json::from_value::<Vec<u64>>(join_roles_config.roles)?;

        event_handler
            .bot
            .http
            .update_guild_member(guild.id, member_add.user.id)
            .roles(
                roles
                    .iter()
                    .map(|id| RoleId::from(*id))
                    .collect::<Vec<RoleId>>()
                    .as_slice(),
            )
            .exec()
            .await?;
    }

    Ok(())
}

pub async fn handle_member_remove(
    member_remove: MemberRemove,
    event_handler: &EventHandler<'_>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let guild = event_handler
        .bot
        .http
        .guild(member_remove.guild_id)
        .exec()
        .await?
        .model()
        .await?;

    let leave_config = event_handler
        .bot
        .db
        .welcome()
        .module_leave_fetch_by_guild_id(guild.id.0)
        .await?;

    if leave_config.enabled {
        let welcome_channel_id = ChannelId(leave_config.channel_id.parse::<u64>()?);

        let channel = event_handler
            .bot
            .http
            .channel(welcome_channel_id)
            .exec()
            .await?
            .model()
            .await?;

        let parsed = parse_message(&leave_config.content, &guild, &member_remove.user, &channel);

        event_handler
            .bot
            .http
            .create_message(welcome_channel_id)
            .content(&parsed)?
            .exec()
            .await?;
    }

    Ok(())
}

async fn member_join_content(
    join_config: WelcomeJoin,
    guild: &Guild,
    user: &User,
    event_handler: &EventHandler<'_>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let welcome_channel_id = ChannelId(join_config.channel_id.parse::<u64>()?);

    let channel = event_handler
        .bot
        .http
        .channel(welcome_channel_id)
        .exec()
        .await?
        .model()
        .await?;

    let parsed = parse_message(&join_config.content, guild, user, &channel);

    event_handler
        .bot
        .http
        .create_message(welcome_channel_id)
        .content(&parsed)?
        .exec()
        .await?;

    Ok(())
}

async fn member_join_embed(
    join_config: WelcomeJoin,
    guild: &Guild,
    user: &User,
    event_handler: &EventHandler<'_>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let welcome_channel_id = ChannelId(join_config.channel_id.parse::<u64>()?);

    let channel = event_handler
        .bot
        .http
        .channel(welcome_channel_id)
        .exec()
        .await?
        .model()
        .await?;

    let embed: twilight_model::channel::embed::Embed =
        serde_json::from_value::<Embed>(join_config.embed)?.into();

    let embed = parse_embed(embed, guild, user, &channel);

    event_handler
        .bot
        .http
        .create_message(welcome_channel_id)
        .embeds(&[embed])?
        .exec()
        .await?;

    Ok(())
}

async fn member_join_dm_content(
    join_dm_config: WelcomeJoinDm,
    guild: &Guild,
    user: &User,
    event_handler: &EventHandler<'_>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let private_channel = event_handler
        .bot
        .http
        .create_private_channel(user.id)
        .exec()
        .await?
        .model()
        .await?;

    let channel = Channel::Private(private_channel);

    let parsed = parse_message(&join_dm_config.content, guild, user, &channel);

    event_handler
        .bot
        .http
        .create_message(channel.id())
        .content(&parsed)?
        .exec()
        .await?;

    Ok(())
}

async fn member_join_dm_embed(
    join_dm_config: WelcomeJoinDm,
    guild: &Guild,
    user: &User,
    event_handler: &EventHandler<'_>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let private_channel = event_handler
        .bot
        .http
        .create_private_channel(user.id)
        .exec()
        .await?
        .model()
        .await?;

    let channel = Channel::Private(private_channel);

    let embed: twilight_model::channel::embed::Embed =
        serde_json::from_value::<Embed>(join_dm_config.embed)?.into();

    let embed = parse_embed(embed, guild, user, &channel);

    event_handler
        .bot
        .http
        .create_message(channel.id())
        .embeds(&[embed])?
        .exec()
        .await?;

    Ok(())
}

fn parse_embed(
    mut embed: twilight_model::channel::embed::Embed,
    guild: &Guild,
    user: &User,
    channel: &Channel,
) -> twilight_model::channel::embed::Embed {
    let parse_string = |s: String| parse_message(s.as_str(), guild, user, channel);
    embed.description = embed.description.map(parse_string);
    embed.title = embed.title.map(parse_string);
    embed.author = embed.author.map(|mut a| {
        a.name = a.name.map(parse_string);
        a
    });
    embed.footer = embed.footer.map(|mut f| {
        f.text = parse_string(f.text);
        f
    });
    embed.fields.iter_mut().for_each(|f| {
        f.name = parse_message(f.name.as_str(), guild, user, channel);
        f.value = parse_message(f.value.as_str(), guild, user, channel);
    });
    embed
}

fn parse_message(message: &str, guild: &Guild, user: &User, channel: &Channel) -> String {
    let mut interpolating = false;
    let mut start_index: usize = 0;
    let mut res = String::new();
    for (i, c) in message.chars().enumerate() {
        match c {
            '{' => {
                interpolating = true;
                start_index = i;
            }
            '}' if interpolating => {
                match &message[start_index..i + 1] {
                    "{channel}" => res += format!("<#{}>", &channel.id()).as_str(),
                    "{channel.id}" => res += &channel.id().to_string(),
                    "{channel.name}" => res += &channel.name().unwrap_or(""),
                    "{channel.type}" => res += &channel.kind().name(),
                    "{server}" | "{server.name}" => res += &guild.name,
                    "{server.icon}" => res += &guild.icon.as_ref().unwrap_or(&"".to_string()),
                    "{server.icon_url}" => {
                        if let Some(icon) = &guild.icon {
                            res += GuildIconUrl(guild.id, icon.to_string()).as_png().as_str();
                        }
                    }
                    "{server.id}" => res += &guild.id.to_string(),
                    "{server.joined_at}" => {
                        res += &guild
                            .joined_at
                            .as_ref()
                            .unwrap_or(&"INVALID DATE".to_string())
                    }
                    "{server.member_count}" => res += &guild.member_count.unwrap().to_string(),
                    "{server.owner}" => res += format!("<@{}>", &guild.owner_id).as_str(),
                    "{server.owner_id}" => res += &guild.owner_id.to_string(),
                    "{server.region}" => res += &guild.preferred_locale,
                    "{server.verification_level}" => {
                        res += &serde_json::to_string(&guild.verification_level)
                            .unwrap_or("0".to_string())
                    }
                    "{user}" | "{user.name}" => res += &user.name,
                    "{user.avatar}" => res += &user.avatar.as_ref().unwrap_or(&"".to_string()),
                    "{user.avatar_url}" => {
                        if let Some(icon) = &user.avatar {
                            res += UserAvatarUrl(user.id, icon.to_string()).as_png().as_str();
                        }
                    }
                    "{user.bot}" => res += &user.bot.to_string(),
                    "{user.discriminator}" => res += &user.discriminator,
                    "{user.id}" => res += &user.id.to_string(),
                    "{user.idname}" => res += format!("<@{}>", &user.id).as_str(),
                    "{user.mention}" => res += format!("<@!{}>", &user.id).as_str(),
                    _ => res += &message[start_index..i + 1],
                };
                interpolating = false;
            }
            _ => {
                if !interpolating {
                    res.push(c);
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use twilight_embed_builder::{EmbedAuthorBuilder, EmbedFieldBuilder, EmbedFooterBuilder};
    use twilight_model::{
        channel::{Channel, PrivateChannel},
        guild::{Guild, SystemChannelFlags},
        id::{ChannelId, GuildId, UserId},
        user::User,
    };

    use crate::modules::welcome::parse_message;

    use super::parse_embed;

    fn get_test_data() -> (Guild, User, Channel) {
        let guild = Guild {
            afk_channel_id: None,
            afk_timeout: 0,
            application_id: None,
            approximate_member_count: None,
            approximate_presence_count: None,
            banner: None,
            channels: vec![],
            default_message_notifications:
                twilight_model::guild::DefaultMessageNotificationLevel::All,
            description: None,
            discovery_splash: None,
            emojis: vec![],
            explicit_content_filter: twilight_model::guild::ExplicitContentFilter::None,
            features: vec![],
            icon: None,
            id: GuildId(0),
            joined_at: None,
            large: false,
            max_members: None,
            max_presences: None,
            max_video_channel_users: None,
            member_count: Some(24),
            members: vec![],
            mfa_level: twilight_model::guild::MfaLevel::None,
            name: "Test Server".to_string(),
            nsfw_level: twilight_model::guild::NSFWLevel::Default,
            owner_id: UserId(0),
            owner: None,
            permissions: None,
            preferred_locale: "US East".to_string(),
            premium_subscription_count: None,
            premium_tier: twilight_model::guild::PremiumTier::None,
            presences: vec![],
            roles: vec![],
            rules_channel_id: None,
            splash: None,
            stage_instances: vec![],
            system_channel_flags: SystemChannelFlags::empty(),
            system_channel_id: None,
            unavailable: false,
            vanity_url_code: None,
            verification_level: twilight_model::guild::VerificationLevel::None,
            voice_states: vec![],
            widget_channel_id: None,
            widget_enabled: None,
            threads: vec![],
        };

        let user = User {
            avatar: None,
            bot: false,
            discriminator: "1234".to_string(),
            email: None,
            flags: None,
            id: UserId(0),
            locale: None,
            mfa_enabled: None,
            name: "Test User".to_string(),
            premium_type: None,
            public_flags: None,
            system: None,
            verified: None,
            accent_color: None,
            banner: None,
        };

        let channel = Channel::Private(PrivateChannel {
            id: ChannelId(0),
            last_message_id: None,
            last_pin_timestamp: None,
            kind: twilight_model::channel::ChannelType::Private,
            recipients: vec![],
        });

        (guild, user, channel)
    }

    #[test]
    fn test_parse_message() {
        let (guild, user, channel) = get_test_data();
        let message =
            "Welcome to {server.name}, {user.name}! You are member #{server.member_count}.";

        let parsed = parse_message(message, &guild, &user, &channel);
        assert_eq!(
            parsed,
            "Welcome to Test Server, Test User! You are member #24."
        )
    }

    #[test]
    fn test_parse_embed() {
        let (guild, user, channel) = get_test_data();

        let embed = twilight_embed_builder::EmbedBuilder::new()
            .title("{user.name} in title")
            .author(EmbedAuthorBuilder::new().name("{user.name}").build())
            .footer(EmbedFooterBuilder::new("{server.name}").build())
            .field(EmbedFieldBuilder::new("{server.name}", "{server.member_count}").build())
            .build()
            .expect("Failed to build test embed");

        let expected = twilight_embed_builder::EmbedBuilder::new()
            .title("Test User in title")
            .author(EmbedAuthorBuilder::new().name("Test User").build())
            .footer(EmbedFooterBuilder::new("Test Server").build())
            .field(EmbedFieldBuilder::new("Test Server", "24").build())
            .build()
            .expect("Failed to build expected embed");

        let embed = parse_embed(embed, &guild, &user, &channel);

        assert_eq!(embed, expected)
    }
}
