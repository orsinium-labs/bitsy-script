use alloc::string::String;
use alloc::string::ToString;
use core::str::Chars;

pub type ID = String;

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

#[derive(Debug, Clone, PartialEq)]
pub enum Tag {
    /// Line break.
    Br,
    /// Page break.
    Pg,
    /// Apply style effect to text.
    Eff(TextEffect),
    /// End the game.
    End,
    /// Print the variable value.
    SayVar(String),
    // say the item quantity in inventory.
    SayItem(String),
    /// Draw tile.
    DrwT(ID),
    /// Draw sprite.
    DrwS(ID),
    /// Draw item.
    DrwI(ID),
    /// Change room's current palette.
    Pal(ID),
    /// Make avatar look like the given sprite.
    Ava(ID),
    /// Move player to the given room.
    Exit(ID, u8, u8),
    /// Unsupported tag.
    Unknown(String, String),
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
        let mut open_tags: u8 = 0;
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
                    if open_tags == 0 && found_letter {
                        self.stash = Some('{');
                        word.pop();
                        break;
                    }
                    open_tags += 1;
                }
                '}' => {
                    if open_tags != 0 {
                        open_tags -= 1;
                        if open_tags == 0 {
                            return Some(parse_tag(&word));
                        } else {
                            found_letter = true
                        }
                    }
                }
                '\t' | '\x0C' | '\r' | ' ' => {
                    if open_tags == 0 && found_letter {
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
    let (name, args) = word.split_once(' ').unwrap_or((word, ""));
    let args = args.trim_ascii();
    if args.is_empty() {
        parse_tag_without_args(name)
    } else {
        parse_tag_with_args(name, args)
    }
}

fn parse_tag_with_args(name: &str, args: &str) -> Tag {
    match name {
        "clr" => match args {
            "0" => Tag::Eff(TextEffect::Color(1)),
            "1" => Tag::Eff(TextEffect::Color(2)),
            "2" => Tag::Eff(TextEffect::Color(3)),
            _ => Tag::Eff(TextEffect::Color(1)),
        },
        "say" | "print" => {
            if let Some(args) = args.strip_prefix("{item") {
                let args = args.strip_suffix('}').unwrap_or(args);
                let args = args.trim_ascii();
                let args = unquote(args);
                Tag::SayItem(args.to_string())
            } else {
                Tag::SayVar(args.to_string())
            }
        }
        "drwt" | "printTile" => Tag::DrwT(unquote(args).to_string()),
        "drws" | "printSprite" => Tag::DrwS(unquote(args).to_string()),
        "drwi" | "printItem" => Tag::DrwI(unquote(args).to_string()),
        "ava" => Tag::Ava(unquote(args).to_string()),
        "pal" => Tag::Pal(unquote(args).to_string()),
        "exit" => {
            let (room, x, y) = parse_exit_args(args);
            let room = room.to_string();
            Tag::Exit(room, x, y)
        }
        _ => Tag::Unknown(name.to_string(), args.to_string()),
    }
}

fn parse_tag_without_args(name: &str) -> Tag {
    match name {
        "br" => Tag::Br,
        "pg" => Tag::Pg,
        "clr1" => Tag::Eff(TextEffect::Color(1)),
        "clr2" => Tag::Eff(TextEffect::Color(2)),
        "clr3" => Tag::Eff(TextEffect::Color(3)),
        "wvy" => Tag::Eff(TextEffect::Wavy),
        "shk" => Tag::Eff(TextEffect::Shaky),
        "rbw" => Tag::Eff(TextEffect::Rainbow),
        "end" => Tag::End,
        _ => Tag::Unknown(name.to_string(), "".to_string()),
    }
}

fn parse_exit_args(args: &str) -> (&str, u8, u8) {
    let args = unquote(args);
    let (room, args) = args.split_once(',').unwrap_or((args, "0,0"));
    let room = unquote(room);
    let (x, y) = args.split_once(',').unwrap_or(("0", "0"));
    let x = x.trim_ascii();
    let y = y.trim_ascii();
    let x: u8 = x.parse().unwrap_or_default();
    let y: u8 = y.parse().unwrap_or_default();
    (room, x, y)
}

fn unquote(v: &str) -> &str {
    if v.starts_with('"') && v.ends_with('"') {
        let v = &v[1..];
        &v[..v.len() - 1]
    } else {
        v
    }
}
