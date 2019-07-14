use lazy_static::lazy_static;
use log::{debug, info};
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

lazy_static! {
    static ref NON_CHAR_REGEX: Regex =
        Regex::new(NON_CHAR_REGEXP).expect("Unable to init non_char_regex!");
    static ref CHANNEL_REGEX: Regex =
        Regex::new(CHANNEL_REGEXP).expect("Unable to init channel_regex!");
}

#[allow(dead_code)]
pub fn is_hangeul(c: char) -> bool {
    let char = c as u32;
    if char >= SYLLABLE_START && char <= SYLLABLE_END {
        return true;
    } else if char >= JAMO_START && char <= JAMO_END {
        return true;
    } else if char >= COMPAT_JAMO_START && char <= COMPAT_JAMO_END {
        return true;
    }
    return false;
}

pub fn strip_content(content: &str) -> Result<String> {
    Ok(NON_CHAR_REGEX.replace_all(content, "").to_string())
}

pub fn parse_content(content: &str) -> Result<(i32, i32, i32)> {
    let mut non_hangeul = 0;
    let mut hangeul = 0;
    let blocks = utils::strip_content(content)?;

    for block in blocks.split("") {
        for character in block.chars() {
            if utils::is_hangeul(character) {
                hangeul += 1;
            } else {
                non_hangeul += 1;
            }
        }
    }

    Ok((hangeul, non_hangeul, non_hangeul + hangeul))
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
