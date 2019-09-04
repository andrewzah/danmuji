use std::error::Error;

use lazy_static::lazy_static;
use log::error;
use rayon::prelude::*;
use regex::Regex;
use serenity::{
    builder::CreateMessage,
    framework::standard::{CommandError, CommandResult},
    model::prelude::*,
    prelude::*,
};

use crate::{models::message::CharCount, errors::Result, utils};

const EMOTE_OR_MENTION_REGEXP: &str = r"<:[\w\d_~-]+:\d+>|<@!\d+>";
const START_LINK_REGEXP: &str = r"^https?://";
const NON_CHAR_REGEXP: &str = r"[^a-zA-Z\p{Hangul}]+";
const CHANNEL_REGEXP: &str = r"<#(?P<i>\d+)>";
const TAG_REGEXP: &str = r">(?P<t>[\w\d\-_]+)";

lazy_static! {
    static ref EMOTE_OR_MENTION_REGEX: Regex =
        Regex::new(EMOTE_OR_MENTION_REGEXP).expect("Unable to init begin_link_regex!");
    static ref START_LINK_REGEX: Regex =
        Regex::new(START_LINK_REGEXP).expect("Unable to init begin_link_regex!");
    static ref NON_CHAR_REGEX: Regex =
        Regex::new(NON_CHAR_REGEXP).expect("Unable to init non_char_regex!");
    static ref CHANNEL_REGEX: Regex =
        Regex::new(CHANNEL_REGEXP).expect("Unable to init channel_regex!");
    static ref TAG_REGEX: Regex = Regex::new(TAG_REGEXP).expect("Unable to init tag_regex!");
}

pub fn starts_with_link(content: &str) -> bool {
    START_LINK_REGEX.is_match(content)
}

pub fn strip_content(content: &str) -> Result<String> {
    Ok(NON_CHAR_REGEX.replace_all(content, "").to_string())
}

pub fn parse_message_content(content: &str) -> Result<CharCount> {
    let stripped = utils::strip_content(content)?;

    let (hangeul_chars, non_hangeul_chars): (Vec<char>, Vec<char>) =
        stripped.par_chars().partition(|c| utils::hangeul::is_hangeul(c));

    let hangeul_count = hangeul_chars.len() as i32;
    let non_hangeul_count = non_hangeul_chars.len() as i32;
    let raw_count = content.chars().collect::<Vec<char>>().len() as i32;

    Ok(CharCount::new(hangeul_count, non_hangeul_count, raw_count))
}

pub fn parse_tag(content: &str) -> Option<&str> {
    TAG_REGEX.captures(content)?.get(1).map(|m| m.as_str())
}

pub fn format_seconds(secs: u64) -> String {
    let weeks = secs / 604800;
    let days = (secs % 604800) / 86400;
    let hours = ((secs % 604800) % 86400) / 3600;
    let minutes = (((secs % 604800) % 86400) % 3600) / 60;
    let seconds = (((secs % 604800) % 86400) % 3600) % 60;

    format!(
        "{}w, {}d, {}h, {}m, {}s",
        weeks, days, hours, minutes, seconds
    )
}

/// Gets rid of `#` in discord input for channel names.
/// ex format) <#500851614362370080> <#500853945342623787>
pub fn format_channels(input: String) -> Result<Vec<String>> {
    Ok(CHANNEL_REGEX
        .replace_all(&input, "$i")
        .split(" ")
        .map(|s| String::from(s))
        .collect())
}

pub fn reply(msg: &Message, ctx: &Context, text: &str) -> CommandResult {
    match msg.reply(ctx, text) {
        Err(err) => {
            error!("Unable to reply: {}", err.description());
            Err(CommandError(err.description().into()))
        },
        _ => Ok(()),
    }
}

pub fn say(channel_id: &ChannelId, ctx: &Context, msg: &str) -> CommandResult {
    match channel_id.say(ctx, msg) {
        Err(err) => {
            error!("Unable to say: {}", err.description());
            Err(CommandError(err.description().into()))
        },
        _ => Ok(()),
    }
}

pub fn send_message<'a, F>(channel_id: &ChannelId, ctx: &Context, f: F) -> CommandResult
where
    for<'b> F: FnOnce(&'b mut CreateMessage<'a>) -> &'b mut CreateMessage<'a>,
{
    match channel_id.send_message(&ctx.http, f) {
        Err(err) => {
            error!("Unable to send message: {}", err.description());
            Err(CommandError(err.description().into()))
        },
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_strips_emoji() {
        assert_eq!("testingpikahuhtest", strip_content("testing <:pikahuh:518007379699302400> test").unwrap());
    }

    #[test]
    fn it_strips_mentions() {
        assert_eq!("testingtest", strip_content("testing <@!167501464414060544> test").unwrap());
    }

    #[test]
    fn it_strips_numbers_and_punctuation() {
        assert_eq!("add", strip_content("add123!@#").unwrap());
    }

    #[test]
    fn it_doesnt_strip_hangul() {
        assert_eq!(
            "addㅇㅁㅏㄴ만",
            strip_content("addㅇㅁㅏㄴ만").unwrap()
        );
    }

    #[test]
    fn it_parses_chars_correctly() {
        assert_eq!(CharCount::new(2, 0, 3), parse_message_content("아침!").unwrap());
        assert_eq!(CharCount::new(2, 2, 4), parse_message_content("ㅁㅁnn").unwrap());
    }

    #[test]
    fn it_parses_chars_with_quotes() {
        assert_eq!(
            CharCount::new(7, 14, 25),
            parse_message_content("'오늘 클립해줄게' lmaooooooooooo").unwrap()
        );
        assert_eq!(
            CharCount::new(7, 14, 25),
            parse_message_content("\"오늘 클립해줄게\" lmaooooooooooo").unwrap()
        );
    }

    #[test]
    fn it_parses_chars_with_punctuation() {
        assert_eq!(
            CharCount::new(0, 23, 28),
            parse_message_content("trev and marvin bucket list:").unwrap()
        );
        assert_eq!(CharCount::new(2, 2, 12), parse_message_content("ㅁ!ㅁ!n @)(*%n").unwrap());
    }

    #[test]
    fn it_parses_in_general() {
        assert_eq!(CharCount::new(0, 10, 16), parse_message_content("50% is accurate.").unwrap());
        assert_eq!(CharCount::new(2, 11, 15), parse_message_content("general test ㅠㅠ").unwrap());

    }

    #[test]
    fn it_parses_max_length_msg() {
        let mut content = String::new();
        for _ in 0..1000 {
            content.push_str("만a");
        }

        assert_eq!(CharCount::new(1000, 1000, 2000), parse_message_content(&content).unwrap());
    }
}
