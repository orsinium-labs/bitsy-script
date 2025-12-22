use crate::*;
use hashbrown::HashMap;

#[derive(Default, Clone, Debug)]
pub struct Inventory {
    items: HashMap<ID, u16>,
}

impl Inventory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn put(&mut self, id: ID) -> u16 {
        *self
            .items
            .entry(id)
            .and_modify(|q| *q = q.saturating_add(1))
            .or_insert(1)
    }

    pub fn pop(&mut self, id: ID) -> u16 {
        *self
            .items
            .entry(id)
            .and_modify(|q| *q = q.saturating_sub(1))
            .or_insert(1)
    }

    pub fn get(&self, id: &ID) -> u16 {
        self.items.get(id).copied().unwrap_or_default()
    }
}
