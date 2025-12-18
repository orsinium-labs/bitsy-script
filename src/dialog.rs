use crate::*;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Dialog {
    pub pages: Vec<Page>,
    pub actions: Vec<Action>,
}

pub struct Page {
    pub lines: Vec<Line>,
}

pub struct Line {
    pub words: Vec<Word>,
}

pub enum Word {
    Text(String, TextEffect),
    Sprite(ID),
    Tile(ID),
    Item(ID),
}
