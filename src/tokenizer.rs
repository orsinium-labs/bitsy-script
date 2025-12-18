use alloc::string::String;
use core::str::Chars;

#[derive(Copy, Clone)]
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

pub(crate) enum Token {
    /// Line break tag.
    TagBr,
    /// Page break tag.
    TagPg,
    /// Text effect tag.
    TagEff(TextEffect),
    /// Unsupported tag.
    TagUnknown,
    /// A closing tag.
    CloseTag,
    /// A plaintext word.
    Word(String),
}

pub(crate) struct Tokenizer<'a> {
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
                '\n' => return Some(Token::TagBr),
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
                        if word.starts_with("{/") {
                            return Some(Token::CloseTag);
                        }
                        let token = match word.as_str() {
                            "{br}" => Token::TagBr,
                            "{pg}" => Token::TagPg,
                            "{clr1}" => Token::TagEff(TextEffect::Color(1)),
                            "{clr2}" => Token::TagEff(TextEffect::Color(2)),
                            "{clr3}" => Token::TagEff(TextEffect::Color(3)),
                            "{clr 1}" => Token::TagEff(TextEffect::Color(1)),
                            "{clr 2}" => Token::TagEff(TextEffect::Color(2)),
                            "{clr 3}" => Token::TagEff(TextEffect::Color(3)),
                            "{wvy}" => Token::TagEff(TextEffect::Wavy),
                            "{shk}" => Token::TagEff(TextEffect::Shaky),
                            "{rbw}" => Token::TagEff(TextEffect::Rainbow),
                            _ => Token::TagUnknown,
                        };
                        return Some(token);
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
