use lazy_static::lazy_static;
use log::{debug, info};
use rayon::prelude::*;
use regex::Regex;

use crate::{
    errors::{AppError, Result},
    utils,
};

const SYLLABLE_START: u32 = 0xAC00;
const SYLLABLE_END: u32 = 0xD7A3;
const JAMO_START: u32 = 0x1100;
const JAMO_END: u32 = 0x11FF;
const COMPAT_JAMO_START: u32 = 0x3130;
const COMPAT_JAMO_END: u32 = 0x318F;

const NON_CHAR_REGEXP: &str = r"[\s\d\W]";
const CHANNEL_REGEXP: &str = r"<#(?P<i>\d+)>";
const TAG_REGEXP: &str = r">(?P<t>[\w\d\-_]+)";

lazy_static! {
    static ref NON_CHAR_REGEX: Regex =
        Regex::new(NON_CHAR_REGEXP).expect("Unable to init non_char_regex!");
    static ref CHANNEL_REGEX: Regex =
        Regex::new(CHANNEL_REGEXP).expect("Unable to init channel_regex!");
    static ref TAG_REGEX: Regex =
        Regex::new(TAG_REGEXP).expect("Unable to init tag_regex!");
}

#[allow(dead_code)]
pub fn is_hangeul(c: char) -> bool {
    let byte = c as u32;
    if byte >= SYLLABLE_START && byte <= SYLLABLE_END {
        return true;
    } else if byte >= JAMO_START && byte <= JAMO_END {
        return true;
    } else if byte >= COMPAT_JAMO_START && byte <= COMPAT_JAMO_END {
        return true;
    }

    return false;
}

pub fn strip_content(content: &str) -> Result<String> {
    Ok(NON_CHAR_REGEX.replace_all(content, "").to_string())
}

pub fn parse_content(content: &str) -> Result<(i32, i32, i32)> {
    let stripped = utils::strip_content(content)?;

    let (hangeul_chars, non_hangeul_chars): (Vec<char>, Vec<char>) =
        stripped.par_chars().partition(|c| is_hangeul(*c));

    let hc = hangeul_chars.len() as i32;
    let nc = non_hangeul_chars.len() as i32;

    Ok((hc, nc, hc + nc))
}

pub fn parse_tag(content: &str) -> Option<&str> {
    TAG_REGEX
        .captures(content)?
        .get(1)
        .map(|m| m.as_str())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_checks_hangeul() {
        assert_eq!(true, is_hangeul('겴'));
        assert_eq!(true, is_hangeul('만'));
        assert_eq!(true, is_hangeul('ㅇ'));
        assert_eq!(true, is_hangeul('ᄓ'));
        assert_eq!(true, is_hangeul('ㄹ'));

    }

    #[test]
    fn it_checks_non_hangeul() {
        assert_eq!(false, is_hangeul('a'));
        assert_eq!(false, is_hangeul('9'));
        assert_eq!(false, is_hangeul('ž'));
    }

    #[test]
    fn it_strips_numbers_and_punctuation() {
        assert_eq!("add", strip_content("add123!@#").unwrap());
    }

    #[test]
    fn it_doesnt_strip_hangul() {
        assert_eq!("addㅇㅁㅏㄴ만", strip_content("addㅇㅁㅏㄴ만").unwrap());
    }

    #[test]
    fn it_parses_chars_correctly() {
        assert_eq!((2, 2, 4), parse_content("ㅁㅁnn").unwrap());
    }

    #[test]
    fn it_parses_chars_with_quotes() {
        assert_eq!((7, 14, 21), parse_content("'오늘 클립해줄게' lmaooooooooooo").unwrap());
        assert_eq!((7, 14, 21), parse_content("\"오늘 클립해줄게\" lmaooooooooooo").unwrap());
    }

    #[test]
    fn it_parses_chars_with_punctuation() {
        assert_eq!((0, 23, 23), parse_content("trev and marvin bucket list:").unwrap());
        assert_eq!((2, 2, 4), parse_content("ㅁ!ㅁ!n @)(*%n").unwrap());
    }

    #[test]
    fn it_parses_max_length_msg() {
        let mut content = String::new();
        for _ in 0..1000 {
            content.push_str("만a");
        }

        assert_eq!((1000, 1000, 2000), parse_content(&content).unwrap());
    }
}
