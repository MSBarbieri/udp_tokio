use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct Player {
    pub id: u8,
    pub position: Position,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub enum Command {
    Walk(i32, i32, i32),
    Shot,
    None,
}
