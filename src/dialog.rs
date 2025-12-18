use crate::*;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Dialog {
    pub pages: Vec<Page>,
}

pub struct Page {
    pub lines: Vec<Line>,
}

pub struct Line {
    pub words: Vec<Word>,
}

pub struct Word {
    pub text: String,
    pub effect: TextEffect,
}
