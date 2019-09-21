use hanja as hanja_util;
use hangeul::is_hangeul;
use serenity::{
    client::Context,
    framework::standard::{
        Args,
        macros::{command, group},
        CommandError, CommandResult,
    },
    model::channel::Message,
    utils::Colour,
};

use crate::utils;

group!({
    name: "hanja",
    options: {
        prefixes: ["hanja", "h"],
        default_command: hanja,
    },
    commands: [
        hanja
    ],
});

#[command]
#[aliases("h")]
fn hanja(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let word = args.single::<String>()?;

    let limit = args.single::<usize>();
    let chars = word.chars().collect::<Vec<char>>();

    let result = match chars.len() {
        1 => get_hanjas(word.chars().collect(), limit.unwrap_or(5)),
        _ => hanja_word(&word, limit.unwrap_or(3))
    };

    if result.len() > word.len() + 5 {
        utils::send_message(&msg.channel_id, &ctx, |m| {
            m.embed(|e| {
                e.colour(Colour::DARK_GOLD);
                e.title("한자");
                e.description(&format!("**{}**:\n{}", word, result));

                e
            });

            m
        })
    } else {
        utils::say(&msg.channel_id, &ctx, "Unable to parse text. Is this 한글?")
    }

}

fn hanja_word(s: &str, limit: usize) -> String {
    let mut result = String::new();

    for character in s.chars() {
        match hanja_util::get(character) {
            Some(c) => {
                result.push(c[0].0)
            },
            None => continue
        }
    }

    result.push_str("\n\n");
    result.push_str(&get_hanjas(s.chars().collect(), limit));

    result
}

fn get_hanjas(chars: Vec<char>, limit: usize) -> String {
    let mut result = String::new();

    for character in &chars {
        match hanja_util::get(*character) {
            Some(c) => {
                for (hanja, desc) in c.iter().take(limit) {
                    result.push_str(&format!("{}: {}\n", hanja, desc));
                }
                result.push_str(&format!("https://hanja.dict.naver.com/search?query={}\n\n", character));
            }
            None => continue
        }
    }

    result
}

