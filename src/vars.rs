use crate::*;
use alloc::string::String;
use hashbrown::HashMap;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Val {
    #[default]
    Undef,
    I(i16),
    S(String),
    F(f32),
}

#[derive(Default)]
pub struct Vars {
    items: HashMap<String, Val>,
}

impl Vars {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, name: String, v: Val) {
        self.items.insert(name, v);
    }

    pub fn get(&self, name: &str) -> &Val {
        self.items.get(name).unwrap_or(&Val::Undef)
    }
}
