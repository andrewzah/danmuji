use rayon::prelude::*;

use crate::errors::Result;

const SYLLABLE_START: u32 = 0xAC00;
const SYLLABLE_END: u32 = 0xD7A3;
const JAMO_START: u32 = 0x1100;
const JAMO_END: u32 = 0x11FF;
const COMPAT_JAMO_START: u32 = 0x3130;
const COMPAT_JAMO_END: u32 = 0x318F;

// TODO error handling?
pub fn char_to_u32(c: &char) -> u32 {
    *c as u32
}

pub fn is_hangeul(c: &char) -> bool {
    let codepoint = char_to_u32(c);

    if codepoint >= SYLLABLE_START && codepoint <= SYLLABLE_END {
        return true;
    } else if codepoint >= JAMO_START && codepoint <= JAMO_END {
        return true;
    } else if codepoint >= COMPAT_JAMO_START && codepoint <= COMPAT_JAMO_END {
        return true;
    }

    return false;
}

enum LeadJamo {
    G,
    GG,
    N,
    D,
    DD,
    R,
    M,
    B,
    BB,
    S,
    SS,
    IEUNG,
    J,
    JJ,
    C,
    K,
    T,
    P,
    H,
}

impl LeadJamo {
    fn from_index(u: u32) -> Option<LeadJamo> {
        match u {
            1  => Some(LeadJamo::G),
            2  => Some(LeadJamo::GG),
            3  => Some(LeadJamo::N),
            4  => Some(LeadJamo::D),
            5  => Some(LeadJamo::DD),
            6  => Some(LeadJamo::R),
            7  => Some(LeadJamo::M),
            8  => Some(LeadJamo::B),
            9  => Some(LeadJamo::BB),
            10 => Some(LeadJamo::S),
            11 => Some(LeadJamo::SS),
            12 => Some(LeadJamo::IEUNG),
            13 => Some(LeadJamo::J),
            14 => Some(LeadJamo::JJ),
            15 => Some(LeadJamo::C),
            16 => Some(LeadJamo::K),
            17 => Some(LeadJamo::T),
            18 => Some(LeadJamo::P),
            19 => Some(LeadJamo::H),
            _ => None,
        }
    }

    fn string(&self) -> String {
        match *self {
            LeadJamo::G     => "ㄱ".into(),
            LeadJamo::GG    => "ㄲ".into(),
            LeadJamo::N     => "ㄴ".into(),
            LeadJamo::D     => "ㄷ".into(),
            LeadJamo::DD    => "ㄸ".into(),
            LeadJamo::R     => "ㄹ".into(),
            LeadJamo::M     => "ㅁ".into(),
            LeadJamo::B     => "ㅂ".into(),
            LeadJamo::BB    => "ㅃ".into(),
            LeadJamo::S     => "ㅅ".into(),
            LeadJamo::SS    => "ㅆ".into(),
            LeadJamo::IEUNG => "ㅇ".into(),
            LeadJamo::J     => "ㅈ".into(),
            LeadJamo::JJ    => "ㅉ".into(),
            LeadJamo::C     => "ㅊ".into(),
            LeadJamo::K     => "ㅋ".into(),
            LeadJamo::T     => "ㅌ".into(),
            LeadJamo::P     => "ㅍ".into(),
            LeadJamo::H     => "ㅎ".into(),
        }
    }
}

enum MiddleJamo {
    A,
    AE,
    YA,
    YAE,
    EO,
    E,
    YEO,
    YE,
    O,
    WA,
    WAE,
    OE,
    YO,
    U,
    WEO,
    WE,
    WI,
    YU,
    EU,
    YI,
    I,
}

impl MiddleJamo {
    fn from_index(u: u32) -> Option<MiddleJamo> {
        match u {
            1  => Some(MiddleJamo::A),
            2  => Some(MiddleJamo::AE),
            3  => Some(MiddleJamo::YA),
            4  => Some(MiddleJamo::YAE),
            5  => Some(MiddleJamo::EO),
            6  => Some(MiddleJamo::E),
            7  => Some(MiddleJamo::YEO),
            8  => Some(MiddleJamo::YE),
            9  => Some(MiddleJamo::O),
            10 => Some(MiddleJamo::WA),
            11 => Some(MiddleJamo::WAE),
            12 => Some(MiddleJamo::OE),
            13 => Some(MiddleJamo::YO),
            14 => Some(MiddleJamo::U),
            15 => Some(MiddleJamo::WEO),
            16 => Some(MiddleJamo::WE),
            17 => Some(MiddleJamo::WI),
            18 => Some(MiddleJamo::YU),
            19 => Some(MiddleJamo::EU),
            20 => Some(MiddleJamo::YI),
            21 => Some(MiddleJamo::I),
            _ => None,
        }
    }

    fn string(&self) -> String {
        match *self {
            MiddleJamo::A   => "ㅏ".into(),
            MiddleJamo::AE  => "ㅐ".into(),
            MiddleJamo::YA  => "ㅑ".into(),
            MiddleJamo::YAE => "ㅒ".into(),
            MiddleJamo::EO  => "ㅓ".into(),
            MiddleJamo::E   => "ㅔ".into(),
            MiddleJamo::YEO => "ㅕ".into(),
            MiddleJamo::YE  => "ㅖ".into(),
            MiddleJamo::O   => "ㅗ".into(),
            MiddleJamo::WA  => "ㅘ".into(),
            MiddleJamo::WAE => "ㅙ".into(),
            MiddleJamo::OE  => "ㅚ".into(),
            MiddleJamo::YO  => "ㅛ".into(),
            MiddleJamo::U   => "ㅜ".into(),
            MiddleJamo::WEO => "ㅝ".into(),
            MiddleJamo::WE  => "ㅞ".into(),
            MiddleJamo::WI  => "ㅟ".into(),
            MiddleJamo::YU  => "ㅠ".into(),
            MiddleJamo::EU  => "ㅡ".into(),
            MiddleJamo::YI  => "ㅢ".into(),
            MiddleJamo::I   => "ㅣ".into(),
        }
    }
}

enum TailJamo {
    G,
    GG,
    GS,
    N,
    NJ,
    NH,
    D,
    L,
    LG,
    LM,
    LB,
    LS,
    LT,
    LP,
    LH,
    M,
    B,
    BS,
    S,
    SS,
    NG,
    J,
    C,
    K,
    T,
    P,
    H
}

impl TailJamo {
    fn from_index(u: u32) -> Option<TailJamo> {
        match u {
            1  => Some(TailJamo::G),
            2  => Some(TailJamo::GG),
            3  => Some(TailJamo::GS),
            4  => Some(TailJamo::N),
            5  => Some(TailJamo::NJ),
            6  => Some(TailJamo::NH),
            7  => Some(TailJamo::D),
            8  => Some(TailJamo::L),
            9  => Some(TailJamo::LG),
            10 => Some(TailJamo::LM),
            11 => Some(TailJamo::LB),
            12 => Some(TailJamo::LS),
            13 => Some(TailJamo::LT),
            14 => Some(TailJamo::LP),
            15 => Some(TailJamo::LH),
            16 => Some(TailJamo::M),
            17 => Some(TailJamo::B),
            18 => Some(TailJamo::BS),
            19 => Some(TailJamo::S),
            20 => Some(TailJamo::SS),
            21 => Some(TailJamo::NG),
            22 => Some(TailJamo::J),
            23 => Some(TailJamo::C),
            24 => Some(TailJamo::K),
            25 => Some(TailJamo::T),
            26 => Some(TailJamo::P),
            27 => Some(TailJamo::H),
            _ => None,
        }
    }

    fn string(&self) -> String {
        match *self {
            TailJamo::G  => "ㄱ".into(),
            TailJamo::GG => "ㄲ".into(),
            TailJamo::GS => "ㄳ".into(),
            TailJamo::N  => "ㄴ".into(),
            TailJamo::NJ => "ㄵ".into(),
            TailJamo::NH => "ㄶ".into(),
            TailJamo::D  => "ㄷ".into(),
            TailJamo::L  => "ㄹ".into(),
            TailJamo::LG => "ㄺ".into(),
            TailJamo::LM => "ㄻ".into(),
            TailJamo::LB => "ㄼ".into(),
            TailJamo::LS => "ㄽ".into(),
            TailJamo::LT => "ㄾ".into(),
            TailJamo::LP => "ㄿ".into(),
            TailJamo::LH => "ㅀ".into(),
            TailJamo::M  => "ㅁ".into(),
            TailJamo::B  => "ㅂ".into(),
            TailJamo::BS => "ㅄ".into(),
            TailJamo::S  => "ㅅ".into(),
            TailJamo::SS => "ㅆ".into(),
            TailJamo::NG => "ㅇ".into(),
            TailJamo::J  => "ㅈ".into(),
            TailJamo::C  => "ㅊ".into(),
            TailJamo::K  => "ㅋ".into(),
            TailJamo::T  => "ㅌ".into(),
            TailJamo::P  => "ㅍ".into(),
            TailJamo::H  => "ㅎ".into(),
        }
    }
}

fn decompose_char(c: &char) -> Vec<String> {
    let mut result = vec![];

    let codepoint = char_to_u32(c);
    let offset = codepoint - 44032;

    let tail = offset % 28;
    let middle = 1 + ((offset - tail) % 588) / 28;
    let lead = 1 + (offset / 588);

    match LeadJamo::from_index(lead) {
        Some(jamo) => result.push(jamo.string()),
        None => {},
    };
    match MiddleJamo::from_index(middle) {
        Some(jamo) => result.push(jamo.string()),
        None => {},
    };
    match TailJamo::from_index(tail) {
        Some(jamo) => result.push(jamo.string()),
        None => {},
    };

    result
}

/// Code point of hangeul:
/// `tail + (vowel - 1) * 28 + (lead - 1) * 588 + 44032`
fn decompose(content: &str) -> Result<String> {
    Ok(
        content
            .chars()
            .collect::<Vec<char>>()
            .par_iter()
            .map(|c| decompose_char(c))
            .flatten()
            .collect()
    )
}

fn compose(contents: Vec<String>) -> Result<String> {
    Ok(
        "todo".into()
    )
}

// TODO; add double consonants
#[derive(Debug)]
enum Hangeul {
    Giyeok,
    Nieun,
    Digeut,
    Rieul,
    Mieum,
    Bieup,
    Sieut,
    Ieung,
    Jieut,
    Chieut,
    Kieuk,
    Tieut,
    Pieup,
    Hieuh,
    A,
    E,
    O,
    I,
    U,
    AE,
    EO,
    EU,
    YO,
    YU,
    YEO,
    YA,
    YE,
    YAE,
}

impl Hangeul {
    pub fn to_ascii(&self) -> char {
        match *self {
            Hangeul::Giyeok => 'r',
            Hangeul::Nieun  => 's',
            Hangeul::Digeut => 'e',
            Hangeul::Rieul  => 'f',
            Hangeul::Mieum  => 'a',
            Hangeul::Bieup  => 'q',
            Hangeul::Sieut  => 't',
            Hangeul::Ieung  => 'd',
            Hangeul::Jieut  => 'w',
            Hangeul::Chieut => 'c',
            Hangeul::Kieuk  => 'z',
            Hangeul::Tieut  => 'x',
            Hangeul::Pieup  => 'v',
            Hangeul::Hieuh  => 'g',
            Hangeul::A      => 'k',
            Hangeul::EO     => 'j',
            Hangeul::O      => 'h',
            Hangeul::I      => 'l',
            Hangeul::U      => 'n',
            Hangeul::EU     => 'm',
            Hangeul::AE     => 'o',
            Hangeul::E      => 'p',
            Hangeul::YO     => 'y',
            Hangeul::YU     => 'B',
            Hangeul::YEO    => 'u',
            Hangeul::YA     => 'i',
            Hangeul::YAE    => 'O',
            Hangeul::YE     => 'P',
        }
    }

    pub fn to_hangeul(&self) -> char {
        match *self {
            Hangeul::Giyeok => 'ㄱ',
            Hangeul::Nieun  => 'ㄴ',
            Hangeul::Digeut => 'ㄷ',
            Hangeul::Rieul  => 'ㄹ',
            Hangeul::Mieum  => 'ㅁ',
            Hangeul::Bieup  => 'ㅂ',
            Hangeul::Sieut  => 'ㅅ',
            Hangeul::Ieung  => 'ㅇ',
            Hangeul::Jieut  => 'ㅈ',
            Hangeul::Chieut => 'ㅊ',
            Hangeul::Kieuk  => 'ㅋ',
            Hangeul::Tieut  => 'ㅌ',
            Hangeul::Pieup  => 'ㅍ',
            Hangeul::Hieuh  => 'ㅎ',
            Hangeul::A      => 'ㅏ',
            Hangeul::EO     => 'ㅓ',
            Hangeul::O      => 'ㅗ',
            Hangeul::I      => 'ㅣ',
            Hangeul::U      => 'ㅜ',
            Hangeul::EU     => 'ㅡ',
            Hangeul::AE     => 'ㅐ',
            Hangeul::E      => 'ㅔ',
            Hangeul::YO     => 'ㅛ',
            Hangeul::YU     => 'ㅠ',
            Hangeul::YEO    => 'ㅕ',
            Hangeul::YA     => 'ㅑ',
            Hangeul::YAE    => 'ㅒ',
            Hangeul::YE     => 'ㅖ',
        }
    }

    pub fn from_ascii(c: &char) -> Option<Hangeul> {
        match c {
             'r' => Some(Hangeul::Giyeok),
             's' => Some(Hangeul::Nieun),
             'e' => Some(Hangeul::Digeut),
             'f' => Some(Hangeul::Rieul),
             'a' => Some(Hangeul::Mieum),
             'q' => Some(Hangeul::Bieup),
             't' => Some(Hangeul::Sieut),
             'd' => Some(Hangeul::Ieung),
             'w' => Some(Hangeul::Jieut),
             'c' => Some(Hangeul::Chieut),
             'z' => Some(Hangeul::Kieuk),
             'x' => Some(Hangeul::Tieut),
             'v' => Some(Hangeul::Pieup),
             'g' => Some(Hangeul::Hieuh),
             'k' => Some(Hangeul::A),
             'j' => Some(Hangeul::EO),
             'h' => Some(Hangeul::O),
             'l' => Some(Hangeul::I),
             'n' => Some(Hangeul::U),
             'm' => Some(Hangeul::EU),
             'o' => Some(Hangeul::AE),
             'p' => Some(Hangeul::E),
             'y' => Some(Hangeul::YO),
             'Y' => Some(Hangeul::YO),
             'b' => Some(Hangeul::YU),
             'u' => Some(Hangeul::YEO),
             'U' => Some(Hangeul::YEO),
             'i' => Some(Hangeul::YA),
             'I' => Some(Hangeul::YEO),
             'O' => Some(Hangeul::YAE),
             'P' => Some(Hangeul::YE),
             _ => None
        }
    }

    pub fn from_hangeul(c: &char) -> Option<Hangeul> {
        println!("c: {}", c);

        match c {
             'ㄱ' => Some(Hangeul::Giyeok),
             'ㄴ' => Some(Hangeul::Nieun),
             'ㄷ' => Some(Hangeul::Digeut),
             'ㄹ' => Some(Hangeul::Rieul),
             'ㅁ' => Some(Hangeul::Mieum),
             'ㅂ' => Some(Hangeul::Bieup),
             'ㅅ' => Some(Hangeul::Sieut),
             'ㅇ' => Some(Hangeul::Ieung),
             'ㅈ' => Some(Hangeul::Jieut),
             'ㅊ' => Some(Hangeul::Chieut),
             'ㅋ' => Some(Hangeul::Kieuk),
             'ㅌ' => Some(Hangeul::Tieut),
             'ㅍ' => Some(Hangeul::Pieup),
             'ㅎ' => Some(Hangeul::Hieuh),
             'ㅏ' => Some(Hangeul::A),
             'ㅓ' => Some(Hangeul::EO),
             'ㅗ' => Some(Hangeul::O),
             'ㅣ' => Some(Hangeul::I),
             'ㅜ' => Some(Hangeul::U),
             'ㅡ' => Some(Hangeul::EU),
             'ㅐ' => Some(Hangeul::AE),
             'ㅔ' => Some(Hangeul::E),
             'ㅛ' => Some(Hangeul::YO),
             'ㅠ' => Some(Hangeul::YU),
             'ㅕ' => Some(Hangeul::YEO),
             'ㅑ' => Some(Hangeul::YA),
             'ㅒ' => Some(Hangeul::YAE),
             'ㅖ' => Some(Hangeul::YE),
             _ => None
        }
    }
}

pub fn hangeul_to_ascii(input: &str) -> String {
    let decomposed = decompose(input)
        .expect("TODO");

    decomposed
        .chars()
        .map(|c| Hangeul::from_hangeul(&c))
        .filter(|o| o.is_some())
        .map(|o| o.unwrap().to_ascii())
        .collect::<Vec<char>>()
        .into_iter()
        .collect()
}

pub fn ascii_to_hangeul(input: &str) -> String {
    let graphemes: Vec<String> = input
        .chars()
        .map(|c| Hangeul::from_ascii(&c))
        .filter(|o| o.is_some())
        .map(|o| o.unwrap().to_hangeul())
        .map(|c| c.to_string())
        .collect();

    compose(graphemes).expect("TODO")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_checks_hangeul() {
        assert_eq!(true, is_hangeul(&'겴'));
        assert_eq!(true, is_hangeul(&'만'));
        assert_eq!(true, is_hangeul(&'ㅇ'));
        assert_eq!(true, is_hangeul(&'ᄓ'));
        assert_eq!(true, is_hangeul(&'ㄹ'));
    }

    #[test]
    fn it_checks_non_hangeul() {
        assert_eq!(false, is_hangeul(&'a'));
        assert_eq!(false, is_hangeul(&'9'));
        assert_eq!(false, is_hangeul(&'ž'));
    }

    #[test]
    fn it_decomposes_hangul() {
        assert_eq!("ㅁㅏ", decompose("마").unwrap());
        assert_eq!("ㅇㅏㄴㄴㅕㅇㅎㅏㅅㅔㅇㅛ", decompose("안녕하세요").unwrap());
    }

    #[test]
    fn it_composes_hangeul() {
        assert_eq!("마", compose("ㅁㅏ"));
        assert_eq!("안녕하세요", compose("ㅇㅏㄴㄴㅕㅇㅎㅏㅅㅔㅇㅛ").unwrap());
    }

    #[test]
    fn it_converts_hangeul_to_ascii() {
        assert_eq!("dkssudgktpdy", hangeul_to_ascii("안녕하세요"));
    }

    #[test]
    fn it_converts_ascii_to_hangeul() {
        assert_eq!("안녕하세요", ascii_to_hangeul("dkssudgktpdy"));
    }
}
