use crate::*;
use alloc::string::String;
use alloc::string::ToString;
use hashbrown::HashMap;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Val {
    #[default]
    Undef,
    I(i16),
    S(String),
    F(f32),
}

impl Val {
    pub fn from_str(&self, s: &str) -> Val {
        let s = s.trim_ascii();
        if s == "true" {
            return Val::I(1);
        }
        if s == "false" {
            return Val::I(0);
        }
        if let Ok(i) = s.parse::<i16>() {
            return Val::I(i);
        }
        if let Ok(f) = s.parse::<f32>() {
            return Val::F(f);
        }
        Val::S(unquote(s).to_string())
    }
}

fn unquote(v: &str) -> &str {
    let n_quotes = v.chars().filter(|ch| *ch == '"').count();
    if n_quotes != 2 {
        return v;
    }
    if v.starts_with('"') && v.ends_with('"') {
        let v = &v[1..];
        &v[..v.len() - 1]
    } else {
        v
    }
}

#[derive(Default, Clone, Debug)]
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
