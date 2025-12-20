use crate::*;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Default)]
pub struct Vars {
    items: Vec<Var>,
}

#[derive(Debug, Clone, PartialEq)]
struct Var {
    name: String,
    val: Val,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    I(i16),
    S(String),
    F(f32),
}

impl Vars {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, name: String, v: Val) -> &Val {
        for item in &mut self.items {
            if item.name == name {
                item.val = match (&item.val, &v) {
                    (Val::I(a), Val::I(b)) => Val::I(a + b),
                    (Val::I(a), Val::F(b)) => Val::F(*a as f32 + b),
                    (Val::F(a), Val::I(b)) => Val::F(a + *b as f32),
                    (Val::S(a), Val::S(b)) => Val::S(alloc::format!("{a}{b}")),
                    (Val::F(a), Val::F(b)) => Val::F(a + b),
                    _ => v,
                };
                return &item.val;
            }
        }
        &Val::I(0)
    }

    pub fn set(&mut self, name: String, v: Val) {
        for item in &mut self.items {
            if item.name == name {
                item.val = v;
                break;
            }
        }
    }

    pub fn get(&self, name: String) -> Option<&Val> {
        for item in &self.items {
            if item.name == name {
                return Some(&item.val);
            }
        }
        None
    }
}
