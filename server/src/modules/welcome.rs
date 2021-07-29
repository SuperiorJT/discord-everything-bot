use std::error::Error;

use twilight_model::{gateway::payload::MemberAdd, guild::Guild, id::ChannelId, user::User};

use crate::bot::event_handler::EventHandler;

// TODO: We need crud for welcome message and welcome channel
// TODO: Create slash commands for setting the above

pub async fn handle_member_add(member_add: Box<MemberAdd>, event_handler: &EventHandler<'_>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let guild = event_handler.bot.http.guild(member_add.guild_id).await?.unwrap();
    
    // TODO: Pull this from db
    let message  = "Welcome to {server.name}, {member.name}! You are member #{server.member_count}.".to_string();

    // TODO: Pull this from db
    let welcome_channel_id = ChannelId(859979803544911872);

    let parsed = parse_message(&message, &guild, &member_add.user);

    event_handler.bot.http.create_message(welcome_channel_id).content(parsed)?.await?;

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