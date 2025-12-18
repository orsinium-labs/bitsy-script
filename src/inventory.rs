use crate::*;
use alloc::vec::Vec;

#[derive(Default)]
pub struct Inventory {
    items: Vec<Item>,
}

struct Item {
    id: ID,
    qty: u8,
}

impl Inventory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn put(&mut self, id: ID) -> u8 {
        for item in &mut self.items {
            if item.id == id {
                item.qty = item.qty.saturating_add(1);
                return item.qty;
            }
        }
        0
    }

    pub fn pop(&mut self, id: ID) -> u8 {
        for item in &mut self.items {
            if item.id == id {
                item.qty = item.qty.saturating_sub(1);
                return item.qty;
            }
        }
        0
    }

    pub fn get(&mut self, id: ID) -> u8 {
        for item in &mut self.items {
            if item.id == id {
                return item.qty;
            }
        }
        0
    }
}
