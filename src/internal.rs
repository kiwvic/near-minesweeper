use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Field {
    pub field: Base64VecU8,
}

impl Field {
    pub fn new(&mut self) -> Self {
        Self {
            field: Base64VecU8::from(vec![0u8; FIELD_LEN]),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct GameInfo {
    pub field: Field,
    pub first_turn_time: Timestamp,
    pub last_turn_time: Timestamp,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub struct PlayerStats {
    pub wins: u32,
    pub losses: u32,

    pub fastest_game: u32,
    pub longest_game: u32,
}
