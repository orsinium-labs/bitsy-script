use crate::*;
use alloc::string::String;

type ID = String;

#[derive(Default, Clone)]
pub struct State {
    pub room: ID,
    pub pos_x: u8,
    pub pos_y: u8,
    pub avatar: ID,
    pub palette: ID,
    pub end: bool,

    pub inventory: Inventory,
    pub vars: Vars,
    pub effect: TextEffect,
}
