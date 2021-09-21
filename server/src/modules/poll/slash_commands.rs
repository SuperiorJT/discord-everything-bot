use std::error::Error;

use chrono::{Local, TimeZone};
use twilight_embed_builder::{EmbedAuthorBuilder, EmbedBuilder, EmbedFooterBuilder};
use twilight_http::request::prelude::RequestReactionType;
use twilight_model::application::interaction::ApplicationCommand;

use crate::{
    bot::{event_handler::EventHandler},
    modules::poll::Poll,
};

pub struct PollCommand<'a>(pub &'a Box<ApplicationCommand>);

impl<'a> PollCommand<'a> {
    pub async fn process_command(
        &self,
        event_handler: &EventHandler<'a>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let command = self.0;

        let poll = Poll::from(command.as_ref());

        let mut embed = EmbedBuilder::new()
            .description(poll.question.clone())
            .color(0xFFC0CB)
            .build()?;

        let mut author = EmbedAuthorBuilder::new()
            .name(poll.author.name.clone())
            .build();
        if let Some(avatar) = poll.author.avatar.clone() {
            author.icon_url.replace(format!(
                "{}{}/{}{}",
                "https://cdn.discordapp.com/avatars/", poll.author.id, avatar, ".png"
            ));
        }

        embed.author.replace(author);

        embed.footer.replace(
            EmbedFooterBuilder::new(match poll.ends {
                Some(ends) => format!(
                    "Vote for Poll! | Ends {}",
                    Local
                        .from_local_datetime(&ends)
                        .unwrap()
                        .format("%b %-d, %Y at %-I:%M%P %Z")
                ),
                None => "Vote for Poll!".into(),
            })
            .build(),
        );

        event_handler
            .embed_interaction_reply(command, vec![embed])
            .await?;

        let original_response = event_handler
            .bot
            .http
            .get_interaction_original(&command.token)?
            .exec()
            .await?
            .model()
            .await?;

        // let (positive, negative) = (
        //     RequestReactionType::Custom {
        //         id: EmojiId(759194715294793778),
        //         name: Some("PeepoYes".into()),
        //     },
        //     RequestReactionType::Custom {
        //         id: EmojiId(759194735314731008),
        //         name: Some("PeepoNo".into()),
        //     },
        // );

        // let db = &event_handler.bot.db;
        // db.poll()
        //     .create_poll(&mut poll)
        //     .await
        //     .expect("could not create poll in db");

        

        // let response;
        // let (positive, negative) = match db.poll().fetch_poll_options(command.guild_id.unwrap()).await {
        //     Ok(res) => {
        //         response = res;
        //         (
        //             RequestReactionType::Custom {
        //                 id: response.0 .0,
        //                 name: Some(&response.0 .1),
        //             },
        //             RequestReactionType::Custom {
        //                 id: response.1 .0,
        //                 name: Some(&response.1 .1),
        //             },
        //         )
        //     },
        //     Err(_) => (
        //         RequestReactionType::Unicode {
        //             name: "👍".into()
        //         },
        //         RequestReactionType::Unicode {
        //             name: "👎".into()
        //         },
        //     ),
        // };

        let positive = RequestReactionType::Unicode {
            name: "👍"
        };
        let negative = RequestReactionType::Unicode {
            name: "👎"
        };


        event_handler
            .bot
            .http
            .create_reaction(original_response.channel_id, original_response.id, &positive)
            .exec()
            .await?;
        event_handler
            .bot
            .http
            .create_reaction(original_response.channel_id, original_response.id, &negative)
            .exec()
            .await?;

        Ok(())
    }
}

pub struct PollEmojiCommand<'a>(pub &'a Box<ApplicationCommand>);

impl<'a> PollEmojiCommand<'a> {
    pub async fn process_command(
        &self,
        event_handler: &EventHandler<'a>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        event_handler.simple_interaction_reply(self.0, "Pong!").await?;

        Ok(())
    }
}