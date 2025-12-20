use crate::*;
use alloc::string::String;

pub enum Word {
    Text(String, TextEffect),
    Sprite(ID),
    Tile(ID),
    Item(ID),
    LineBreak,
    PageBreak,
}

pub struct Interpreter<'a, T: Iterator<Item = Token>> {
    pub tokens: T,
    pub state: &'a mut State,
}

impl<'a, T: Iterator<Item = Token>> Iterator for Interpreter<'a, T> {
    type Item = Word;

    fn next(&mut self) -> Option<Self::Item> {
        interpret(&mut self.tokens, self.state)
    }
}

pub fn interpret<T>(tokens: &mut T, state: &mut State) -> Option<Word>
where
    T: Iterator<Item = Token>,
{
    for token in tokens.by_ref() {
        match token {
            Token::OpenTag(tag) => todo!(),
            Token::CloseTag(tag) => todo!(),
            Token::Word(t) => return Some(Word::Text(t, state.effect)),
        }
    }
    None
}
