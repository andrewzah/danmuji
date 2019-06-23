use regex::Regex;

const SYLLABLE_START: u32 = 0xAC00;
const SYLLABLE_END: u32 = 0xD7A3;

const JAMO_START: u32 = 0x1100;
const JAMO_END: u32 = 0x11FF;

const COMPAT_JAMO_START: u32 = 0x3130;
const COMPAT_JAMO_END: u32 = 0x318F;

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

const REGEX: &str = r"[\s\d\W]";

pub fn format_content(content: &str) -> String {
    let re = Regex::new(REGEX).expect("Unable to create regex instance!");
    re.replace_all(content, "").to_string()
}
