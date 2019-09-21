use hangeul::{is_compat_jamo, decompose_char, compose_char};

use crate::errors::Result;

/// Counts the number of Jamo in a string of Hangeul characters.
pub fn jamo_length(input: &str) -> Result<i32> {
    Ok(0)
}

pub fn hangeul_to_ascii(input: &str) -> String {
    let mut chars: Vec<char> = vec![];

    for c in input.chars() {
        println!("c: {}", &c);

        if is_compat_jamo(c as u32) {
            chars.push(c);
        } else {
            let (cho, jung, jong) = match decompose_char(&c) {
                Ok((cho, jung, jong)) => (cho, jung, jong),
                Err(_) => continue,
            };

            chars.push(cho);
            chars.push(jung);

            if let Some(jongseong) = jong {
                chars.push(jongseong)
            }
        }
    }

    println!("chars: {:?}", &chars);

    chars
        .into_iter()
        .map(|c| HangeulAsciiMap::from_compat_jamo(&c))
        .filter(|o| o.is_some())
        .map(|o| o.unwrap().to_ascii())
        .collect::<Vec<char>>()
        .into_iter()
        .collect()
}

pub fn ascii_to_hangeul(input: &str) -> String {
    let graphemes: String = input
        .chars()
        .map(|c| HangeulAsciiMap::from_ascii(&c))
        .filter(|option| option.is_some())
        .map(|option| option.unwrap().to_compat_jamo())
        .collect();

    //compose(&graphemes)
    graphemes
}


// TODO; add double consonants
#[derive(Debug)]
enum HangeulAsciiMap {
    SsangGiyeok,
    Giyeok,
    Nieun,
    SsangDigeut,
    Digeut,
    Rieul,
    Mieum,
    SsangBieup,
    Bieup,
    SsangSiot,
    Siot,
    Ieung,
    SsangJieut,
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

impl HangeulAsciiMap {
    pub fn to_ascii(&self) -> char {
        match *self {
            HangeulAsciiMap::SsangGiyeok => 'R',
            HangeulAsciiMap::Giyeok => 'r',
            HangeulAsciiMap::Nieun  => 's',
            HangeulAsciiMap::SsangDigeut => 'E',
            HangeulAsciiMap::Digeut => 'e',
            HangeulAsciiMap::Rieul  => 'f',
            HangeulAsciiMap::Mieum  => 'a',
            HangeulAsciiMap::SsangBieup => 'Q',
            HangeulAsciiMap::Bieup  => 'q',
            HangeulAsciiMap::SsangSiot  => 'T',
            HangeulAsciiMap::Siot  => 't',
            HangeulAsciiMap::Ieung  => 'd',
            HangeulAsciiMap::SsangJieut  => 'W',
            HangeulAsciiMap::Jieut  => 'w',
            HangeulAsciiMap::Chieut => 'c',
            HangeulAsciiMap::Kieuk  => 'z',
            HangeulAsciiMap::Tieut  => 'x',
            HangeulAsciiMap::Pieup  => 'v',
            HangeulAsciiMap::Hieuh  => 'g',
            HangeulAsciiMap::A      => 'k',
            HangeulAsciiMap::EO     => 'j',
            HangeulAsciiMap::O      => 'h',
            HangeulAsciiMap::I      => 'l',
            HangeulAsciiMap::U      => 'n',
            HangeulAsciiMap::EU     => 'm',
            HangeulAsciiMap::AE     => 'o',
            HangeulAsciiMap::E      => 'p',
            HangeulAsciiMap::YO     => 'y',
            HangeulAsciiMap::YU     => 'B',
            HangeulAsciiMap::YEO    => 'u',
            HangeulAsciiMap::YA     => 'i',
            HangeulAsciiMap::YAE    => 'O',
            HangeulAsciiMap::YE     => 'P',
        }
    }

    pub fn to_compat_jamo(&self) -> char {
        match *self {
            HangeulAsciiMap::SsangGiyeok => 'ㄲ',
            HangeulAsciiMap::Giyeok => 'ㄱ',
            HangeulAsciiMap::Nieun  => 'ㄴ',
            HangeulAsciiMap::SsangDigeut => 'ㄸ',
            HangeulAsciiMap::Digeut => 'ㄷ',
            HangeulAsciiMap::Rieul  => 'ㄹ',
            HangeulAsciiMap::Mieum  => 'ㅁ',
            HangeulAsciiMap::SsangBieup  => 'ㅃ',
            HangeulAsciiMap::Bieup  => 'ㅂ',
            HangeulAsciiMap::SsangSiot  => 'ㅆ',
            HangeulAsciiMap::Siot  => 'ㅅ',
            HangeulAsciiMap::Ieung  => 'ㅇ',
            HangeulAsciiMap::SsangJieut  => 'ㅉ',
            HangeulAsciiMap::Jieut  => 'ㅈ',
            HangeulAsciiMap::Chieut => 'ㅊ',
            HangeulAsciiMap::Kieuk  => 'ㅋ',
            HangeulAsciiMap::Tieut  => 'ㅌ',
            HangeulAsciiMap::Pieup  => 'ㅍ',
            HangeulAsciiMap::Hieuh  => 'ㅎ',
            HangeulAsciiMap::A      => 'ㅏ',
            HangeulAsciiMap::EO     => 'ㅓ',
            HangeulAsciiMap::O      => 'ㅗ',
            HangeulAsciiMap::I      => 'ㅣ',
            HangeulAsciiMap::U      => 'ㅜ',
            HangeulAsciiMap::EU     => 'ㅡ',
            HangeulAsciiMap::AE     => 'ㅐ',
            HangeulAsciiMap::E      => 'ㅔ',
            HangeulAsciiMap::YO     => 'ㅛ',
            HangeulAsciiMap::YU     => 'ㅠ',
            HangeulAsciiMap::YEO    => 'ㅕ',
            HangeulAsciiMap::YA     => 'ㅑ',
            HangeulAsciiMap::YAE    => 'ㅒ',
            HangeulAsciiMap::YE     => 'ㅖ',
        }
    }

    pub fn from_ascii(c: &char) -> Option<HangeulAsciiMap> {
        match c {
             'R' => Some(HangeulAsciiMap::SsangGiyeok),
             'r' => Some(HangeulAsciiMap::Giyeok),
             's' => Some(HangeulAsciiMap::Nieun),
             'E' => Some(HangeulAsciiMap::SsangDigeut),
             'e' => Some(HangeulAsciiMap::Digeut),
             'f' => Some(HangeulAsciiMap::Rieul),
             'a' => Some(HangeulAsciiMap::Mieum),
             'Q' => Some(HangeulAsciiMap::SsangBieup),
             'q' => Some(HangeulAsciiMap::Bieup),
             'T' => Some(HangeulAsciiMap::SsangSiot),
             't' => Some(HangeulAsciiMap::Siot),
             'd' => Some(HangeulAsciiMap::Ieung),
             'W' => Some(HangeulAsciiMap::SsangJieut),
             'w' => Some(HangeulAsciiMap::Jieut),
             'c' => Some(HangeulAsciiMap::Chieut),
             'z' => Some(HangeulAsciiMap::Kieuk),
             'x' => Some(HangeulAsciiMap::Tieut),
             'v' => Some(HangeulAsciiMap::Pieup),
             'g' => Some(HangeulAsciiMap::Hieuh),
             'k' => Some(HangeulAsciiMap::A),
             'j' => Some(HangeulAsciiMap::EO),
             'h' => Some(HangeulAsciiMap::O),
             'l' => Some(HangeulAsciiMap::I),
             'n' => Some(HangeulAsciiMap::U),
             'm' => Some(HangeulAsciiMap::EU),
             'o' => Some(HangeulAsciiMap::AE),
             'p' => Some(HangeulAsciiMap::E),
             'y' => Some(HangeulAsciiMap::YO),
             'Y' => Some(HangeulAsciiMap::YO),
             'b' => Some(HangeulAsciiMap::YU),
             'u' => Some(HangeulAsciiMap::YEO),
             'U' => Some(HangeulAsciiMap::YEO),
             'i' => Some(HangeulAsciiMap::YA),
             'I' => Some(HangeulAsciiMap::YEO),
             'O' => Some(HangeulAsciiMap::YAE),
             'P' => Some(HangeulAsciiMap::YE),
             _ => None
        }
    }

    pub fn from_compat_jamo(c: &char) -> Option<HangeulAsciiMap> {
        match c {
             'ㄲ' => Some(HangeulAsciiMap::SsangGiyeok),
             'ㄱ' => Some(HangeulAsciiMap::Giyeok),
             'ㄴ' => Some(HangeulAsciiMap::Nieun),
             'ㄸ' => Some(HangeulAsciiMap::SsangDigeut),
             'ㄷ' => Some(HangeulAsciiMap::Digeut),
             'ㄹ' => Some(HangeulAsciiMap::Rieul),
             'ㅁ' => Some(HangeulAsciiMap::Mieum),
             'ㅃ' => Some(HangeulAsciiMap::SsangBieup),
             'ㅂ' => Some(HangeulAsciiMap::Bieup),
             'ㅆ' => Some(HangeulAsciiMap::SsangSiot),
             'ㅅ' => Some(HangeulAsciiMap::Siot),
             'ㅇ' => Some(HangeulAsciiMap::Ieung),
             'ㅉ' => Some(HangeulAsciiMap::SsangJieut),
             'ㅈ' => Some(HangeulAsciiMap::Jieut),
             'ㅊ' => Some(HangeulAsciiMap::Chieut),
             'ㅋ' => Some(HangeulAsciiMap::Kieuk),
             'ㅌ' => Some(HangeulAsciiMap::Tieut),
             'ㅍ' => Some(HangeulAsciiMap::Pieup),
             'ㅎ' => Some(HangeulAsciiMap::Hieuh),
             'ㅏ' => Some(HangeulAsciiMap::A),
             'ㅓ' => Some(HangeulAsciiMap::EO),
             'ㅗ' => Some(HangeulAsciiMap::O),
             'ㅣ' => Some(HangeulAsciiMap::I),
             'ㅜ' => Some(HangeulAsciiMap::U),
             'ㅡ' => Some(HangeulAsciiMap::EU),
             'ㅐ' => Some(HangeulAsciiMap::AE),
             'ㅔ' => Some(HangeulAsciiMap::E),
             'ㅛ' => Some(HangeulAsciiMap::YO),
             'ㅠ' => Some(HangeulAsciiMap::YU),
             'ㅕ' => Some(HangeulAsciiMap::YEO),
             'ㅑ' => Some(HangeulAsciiMap::YA),
             'ㅒ' => Some(HangeulAsciiMap::YAE),
             'ㅖ' => Some(HangeulAsciiMap::YE),
             _ => None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn it_composes_hangeul() {
        //assert_eq!("마", compose("ㅁㅏ"));
        //assert_eq!("안녕하세요", compose("ㅇㅏㄴㄴㅕㅇㅎㅏㅅㅔㅇㅛ"));
    //}

    #[test]
    fn it_converts_hangeul_to_ascii() {
        assert_eq!("lol", hangeul_to_ascii("ㅣㅐㅣ"));
        assert_eq!("EhrqhRdl", hangeul_to_ascii("똑볶이"));
        assert_eq!("dkssudgktpdy", hangeul_to_ascii("안녕하세요"));
    }

    #[test]
    fn it_converts_ascii_to_hangeul() {
        assert_eq!("ㅣㅐㅣ", ascii_to_hangeul("lol"));
        assert_eq!("안녕하세요", ascii_to_hangeul("dkssudgktpdy"));
    }
}
