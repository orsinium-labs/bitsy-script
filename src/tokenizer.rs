use alloc::string::String;
use core::str::Chars;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextEffect {
    /// No effects.
    None,
    /// {wvy} text in tags waves up and down.
    Wavy,
    /// {shk} text in tags shakes constantly.
    Shaky,
    /// {rbw} text in tags is rainbow colored.
    Rainbow,
    /// {clr} use a palette color for dialog text.
    Color(u8),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tag {
    /// Line break tag.
    Br,
    /// Page break tag.
    Pg,
    /// Text effect tag.
    Eff(TextEffect),
    /// Unsupported tag.
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenTag(Tag),
    CloseTag(Tag),
    Word(String),
}

pub struct Tokenizer<'a> {
    buffer: Chars<'a>,
    stash: Option<char>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            buffer: text.chars(),
            stash: None,
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut word = String::new();
        let mut found_letter = false;
        let mut inside_tag = false;
        loop {
            let ch = if let Some(stash) = self.stash.take() {
                stash
            } else if let Some(ch) = self.buffer.next() {
                ch
            } else {
                break;
            };
            word.push(ch);
            match ch {
                '\n' => return Some(Token::OpenTag(Tag::Br)),
                '{' => {
                    if found_letter {
                        self.stash = Some('{');
                        word.pop();
                        break;
                    }
                    inside_tag = true;
                }
                '}' => {
                    if inside_tag {
                        return Some(parse_tag(&word));
                    }
                    found_letter = true
                }
                '\t' | '\x0C' | '\r' | ' ' => {
                    if !inside_tag && found_letter {
                        break;
                    }
                }
                _ => found_letter = true,
            }
        }
        if word.is_empty() {
            return None;
        }
        Some(Token::Word(word))
    }
}

fn parse_tag(word: &str) -> Token {
    let word = &word[..word.len() - 1]; // remove "}" from the end.
    let mut word = &word[1..]; // remove "{" from the beginning.
    word = word.trim_ascii();
    let is_closing = word.starts_with('/');
    if is_closing {
        word = &word[1..];
        word = word.trim_ascii();
    }
    let tag = parse_tag_value(word);
    if is_closing {
        Token::CloseTag(tag)
    } else {
        Token::OpenTag(tag)
    }
}

fn parse_tag_value(word: &str) -> Tag {
    match word {
        "br" => Tag::Br,
        "pg" => Tag::Pg,
        "clr1" => Tag::Eff(TextEffect::Color(1)),
        "clr2" => Tag::Eff(TextEffect::Color(2)),
        "clr3" => Tag::Eff(TextEffect::Color(3)),
        "clr 0" => Tag::Eff(TextEffect::Color(1)),
        "clr 1" => Tag::Eff(TextEffect::Color(2)),
        "clr 2" => Tag::Eff(TextEffect::Color(3)),
        "wvy" => Tag::Eff(TextEffect::Wavy),
        "shk" => Tag::Eff(TextEffect::Shaky),
        "rbw" => Tag::Eff(TextEffect::Rainbow),
        _ => Tag::Unknown,
    }
}
