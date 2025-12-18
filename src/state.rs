use crate::*;
use alloc::string::String;

type ID = String;

pub enum Action {
    SetPalette(ID),
    SetAvatar(ID),
    Exit(ID, u8, u8),
    End,
}

pub struct State {
    pub room: ID,
    pub pos_x: u8,
    pub pos_y: u8,
    pub avatar: ID,
    pub inventory: Inventory,
    pub vars: Vars,
}
