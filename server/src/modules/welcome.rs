use std::error::Error;

use twilight_model::{gateway::payload::MemberAdd, guild::Guild, id::ChannelId, user::User};

use crate::{bot::event_handler::EventHandler, db::queries::welcome::WelcomeModule, models::embed::Embed};

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
    pub leave: Option<WelcomeLeaveContent>
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
            leave: None
        };
        if w.join_enabled.is_some() {
            val.join = Some(WelcomeJoinContent {
                enabled: w.join_enabled,
                message_type: w.join_message_type,
                channel_id: w.join_channel_id,
                content: w.join_content,
                embed: w.join_embed.and_then(|val| serde_json::from_str(&val).ok())
            });
        }
        if w.join_dm_enabled.is_some() {
            val.join_dm = Some(WelcomeJoinDmContent {
                enabled: w.join_dm_enabled,
                message_type: w.join_dm_message_type,
                content: w.join_dm_content,
                embed: w.join_dm_embed.and_then(|val| serde_json::from_str(&val).ok())
            });
        }
        if w.join_roles_enabled.is_some() {
            val.join_roles = Some(WelcomeJoinRolesContent {
                enabled: w.join_roles_enabled
            });
        }
        if w.leave_enabled.is_some() {
            val.leave = Some(WelcomeLeaveContent {
                enabled: w.leave_enabled
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
    pub leave_enabled: Option<bool>
}

#[derive(sqlx::FromRow, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeContent {
    pub enabled: Option<bool>
}

#[derive(sqlx::FromRow, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeJoin {
    pub id: i64,
    pub enabled: bool,
    pub message_type: String,
    pub channel_id: String,
    pub content: String,
    pub embed: serde_json::Value
}

#[derive(sqlx::FromRow, serde::Serialize, sqlx::Type, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeJoinContent {
    pub enabled: Option<bool>,
    pub message_type: Option<String>,
    pub channel_id: Option<String>,
    pub content: Option<String>,
    pub embed: Option<serde_json::Value>
}

#[derive(sqlx::FromRow, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeJoinDm {
    pub id: i64,
    pub enabled: bool,
    pub message_type: String,
    pub content: String,
    pub embed: serde_json::Value
}

#[derive(sqlx::FromRow, serde::Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeJoinDmContent {
    pub enabled: Option<bool>,
    pub message_type: Option<String>,
    pub content: Option<String>,
    pub embed: Option<serde_json::Value>
}

#[derive(sqlx::FromRow, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeJoinRoles {
    pub id: i64,
    pub enabled: bool
}

#[derive(sqlx::FromRow, serde::Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeJoinRolesContent {
    pub enabled: Option<bool>
}

#[derive(sqlx::FromRow, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeLeave {
    pub id: i64,
    pub enabled: bool
}
#[derive(sqlx::FromRow, serde::Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeLeaveContent {
    pub enabled: Option<bool>
}

pub async fn handle_member_add(member_add: Box<MemberAdd>, event_handler: &EventHandler<'_>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let guild = event_handler.bot.http.guild(member_add.guild_id).exec().await?.model().await?;

    let join_config = event_handler.bot.db.welcome().module_join_fetch_by_guild_id(guild.id.0).await?;
    
    match join_config.message_type.as_str() {
        "text" => {
            member_add_content(join_config, &guild, &member_add.user, event_handler).await?;
        },
        "embed" => {
            member_add_embed(join_config, &guild, &member_add.user, event_handler).await?;
        }
        _ => {
            return Err("Invalid mesage type on welcome_join".into());
        }
    }

    Ok(())
}

async fn member_add_content(join_config: WelcomeJoin, guild: &Guild, user: &User, event_handler: &EventHandler<'_>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let parsed = parse_message(&join_config.content, guild, user);

    let welcome_channel_id = ChannelId(join_config.channel_id.parse::<u64>()?);

    event_handler.bot.http.create_message(welcome_channel_id).content(&parsed)?.exec().await?;
    
    Ok(())
}

async fn member_add_embed(join_config: WelcomeJoin, guild: &Guild, user: &User, event_handler: &EventHandler<'_>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let welcome_channel_id = ChannelId(join_config.channel_id.parse::<u64>()?);

    let mut embed: twilight_model::channel::embed::Embed = serde_json::from_value::<Embed>(join_config.embed)?.into();

    if let Some(desc) = embed.description {
        embed.description = Some(parse_message(desc.as_str(), guild, user));
    }

    println!("{:?}", embed);

    event_handler.bot.http.create_message(welcome_channel_id).embeds(&[embed])?.exec().await?;
    
    Ok(())
}

fn parse_message(message: &str, guild: &Guild, user: &User) -> String {
    let mut interpolating = false;
    let mut start_index: usize = 0;
    let mut res = String::new();
    for (i, c) in message.chars().enumerate() {
        match c {
            '{' => {
                interpolating = true;
                start_index = i;
            },
            '}' if interpolating => {
                match &message[start_index..i + 1] {
                    "{server.name}" => res.push_str(&guild.name),
                    "{server.member_count}" => res.push_str(&guild.member_count.unwrap().to_string()),
                    "{member.name}" => res.push_str(&user.name),
                    _ => res.push_str(&message[start_index..i + 1])
                };
                interpolating = false;
            },
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
    use twilight_model::{guild::{Guild, SystemChannelFlags}, id::{GuildId, UserId}, user::User};

    use crate::modules::welcome::parse_message;

    #[test]
    fn test_parse_message() {
        let message = "Welcome to {server.name}, {member.name}! You are member #{server.member_count}.";
    
        let guild = Guild {
            afk_channel_id: None,
            afk_timeout: 0,
            application_id: None,
            approximate_member_count: None,
            approximate_presence_count: None,
            banner: None,
            channels: vec![],
            default_message_notifications: twilight_model::guild::DefaultMessageNotificationLevel::All,
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
            name: "The Test Server".to_string(),
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
        };
    
        let parsed = parse_message(message, &guild, &user);
        assert_eq!(parsed, "Welcome to The Test Server, Test User! You are member #24.")
    }
}