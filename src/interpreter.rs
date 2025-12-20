use crate::*;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;

#[derive(Debug, PartialEq)]
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
        Tag::Say(expr) => {
            let val = eval_expr(expr, state);
            let s = val_to_string(&val);
            return Some(Word::Text(s, state.effect));
        }
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
        Expr::SimpleExpr(expr) => eval_simple_expr(expr, state),
        Expr::BinOp(op, lhs, rhs) => {
            let lhs = eval_simple_expr(lhs, state);
            let rhs = eval_simple_expr(rhs, state);
            eval_bin_op(op, lhs, rhs)
        }
    }
}

fn eval_simple_expr(expr: SimpleExpr, state: &mut State) -> Val {
    match expr {
        SimpleExpr::Var(name) => state.vars.get(&name).clone(),
        SimpleExpr::Item(name) => Val::I(state.inventory.get(&name) as i16),
        SimpleExpr::Val(val) => val,
    }
}

fn eval_bin_op(op: BinOp, lhs: Val, rhs: Val) -> Val {
    match op {
        BinOp::Mul => todo!(),
        BinOp::Div => todo!(),
        BinOp::Add => match (lhs, rhs) {
            (Val::I(a), Val::I(b)) => Val::I(a + b),
            (Val::I(a), Val::F(b)) => Val::F(a as f32 + b),
            (Val::F(a), Val::I(b)) => Val::F(a + b as f32),
            (Val::S(a), Val::S(b)) => Val::S(alloc::format!("{a}{b}")),
            (Val::F(a), Val::F(b)) => Val::F(a + b),
            (a, Val::Undef) => a,
            (_, b) => b,
        },
        BinOp::Sub => match (lhs, rhs) {
            (Val::I(a), Val::I(b)) => Val::I(a - b),
            (Val::I(a), Val::F(b)) => Val::F(a as f32 - b),
            (Val::F(a), Val::I(b)) => Val::F(a - b as f32),
            (Val::F(a), Val::F(b)) => Val::F(a + b),
            (Val::Undef, b) => b,
            (a, _) => a,
        },
        BinOp::Lt => todo!(),
        BinOp::Gt => todo!(),
        BinOp::Lte => todo!(),
        BinOp::Gte => todo!(),
        BinOp::Eq => todo!(),
    }
}

fn val_to_string(val: &Val) -> String {
    match val {
        Val::Undef => "0".to_string(),
        Val::I(i) => format!("{i}"),
        Val::S(s) => s.to_string(),
        Val::F(f) => format!("{f}"),
    }
}
