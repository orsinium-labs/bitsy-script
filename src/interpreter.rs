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
        let maybe_word = match token {
            Token::OpenTag(tag) => handle_open_tag(tag, state),
            Token::CloseTag(tag) => handle_close_tag(tag, state),
            Token::Word(t) => Some(Word::Text(t, state.effect)),
        };
        if let Some(word) = maybe_word {
            return Some(word);
        }
    }
    None
}

fn handle_open_tag(tag: Tag, state: &mut State) -> Option<Word> {
    match tag {
        Tag::Br => return Some(Word::LineBreak),
        Tag::Pg => return Some(Word::PageBreak),
        Tag::Eff(eff) => state.effect = eff,
        Tag::End => state.end = true,
        Tag::SayVar(_) => todo!(),
        Tag::SayItem(_) => todo!(),
        Tag::DrwT(_) => todo!(),
        Tag::DrwS(_) => todo!(),
        Tag::DrwI(_) => todo!(),
        Tag::Pal(_) => todo!(),
        Tag::Ava(_) => todo!(),
        Tag::Exit(room, x, y) => {
            state.room = room;
            state.pos_x = x;
            state.pos_y = y;
        }
        Tag::Set(name, expr) => {
            let val = eval_expr(expr, state);
            state.vars.set(name, val);
        }
        Tag::Unknown(_, _) => {}
    };
    None
}

fn handle_close_tag(tag: Tag, state: &mut State) -> Option<Word> {
    if let Tag::Eff(_) = tag {
        state.effect = TextEffect::None
    };
    None
}

fn eval_expr(expr: Expr, state: &mut State) -> Val {
    match expr {
        Expr::SimpleExpr(ex) => todo!(),
        Expr::Mul(ex, ex1) => todo!(),
        Expr::Div(ex, ex1) => todo!(),
        Expr::Add(ex, ex1) => todo!(),
        Expr::Sub(ex, ex1) => todo!(),
        Expr::Lt(ex, ex1) => todo!(),
        Expr::Gt(ex, ex1) => todo!(),
        Expr::Lte(ex, ex1) => todo!(),
        Expr::Gte(ex, ex1) => todo!(),
        Expr::Eq(ex, ex1) => todo!(),
    }
}
