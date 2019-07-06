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

const REGEX: &str = r"[\s\d\W]";

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
    let re = Regex::new(REGEX)?;
    Ok(re.replace_all(content, "").to_string())
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
