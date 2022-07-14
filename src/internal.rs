use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Game {
    pub field: Field,
    pub first_turn_time: Timestamp,
    pub last_turn_time: Timestamp,
}

impl Game {
    pub fn new() -> Self {
        Self {
            field: Field::new(),
            first_turn_time: env::block_timestamp(),
            last_turn_time: 0 as Timestamp
        }
    }
}

#[derive(BorshStorageKey, BorshSerialize)]
pub struct PlayerStats {
    pub wins: u32,
    pub losses: u32,

    pub fastest_game: u32,
    pub longest_game: u32,
}
