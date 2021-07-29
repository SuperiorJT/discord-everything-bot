use chrono::{NaiveDateTime, Utc};
use twilight_model::{application::interaction::{application_command::CommandDataOption, ApplicationCommand}, id::EmojiId, user::User};

pub mod slash_commands;

pub struct Poll {
    pub id: i64,
    pub author: User,
    pub question: String,
    pub start: Option<NaiveDateTime>,
    pub ends: Option<NaiveDateTime>,
    pub options: Option<Vec<PollOption>>,
}

impl From<&ApplicationCommand> for Poll {
    fn from(command: &ApplicationCommand) -> Self {
        let author = command.member.clone().unwrap().user.unwrap();
        let (question, start, ends, options) = command.data.options.iter().fold(
            ("<Unknown Question>".to_string(), None, None, None),
            |mut res, option| {
                match option.name() {
                    "question" => match option {
                        CommandDataOption::String { value, .. } => res.0 = value.clone(),
                        _ => {}
                    },
                    "ends" => match option {
                        CommandDataOption::String { value, .. } => {
                            res.1 = Some(Utc::now().naive_utc());
                            let date = date_time_parser::DateParser::parse(value);
                            let time = date_time_parser::TimeParser::parse(value);
                            match (date, time) {
                                (None, None) => res.1 = None,
                                (None, Some(time)) => {
                                    let date = Utc::now().naive_utc().date();
                                    res.2 = Some(date.and_time(time));
                                }
                                (Some(date), None) => res.2 = Some(date.and_hms(0, 0, 0)),
                                (Some(date), Some(time)) => res.2 = Some(date.and_time(time)),
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
                res
            },
        );

        Self {
            id: 0,
            author,
            question,
            start,
            ends,
            options,
        }
    }
}

pub struct PollOption {
    emoji: EmojiId,
}
