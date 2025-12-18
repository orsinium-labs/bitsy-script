use alloc::string::String;

type ID = String;

pub enum Action {
    DrwTile(ID),
    DrwSprite(ID),
    DrwItem(ID),
    SetPalette(ID),
    SetAvatar(ID),
    Exit(ID, u8, u8),
    End,
}
