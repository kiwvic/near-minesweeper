use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Field {
    pub field: Base64VecU8,
}

impl Field {
    pub fn new() -> Self {
        Self {
            field: Field::init_field(),
        }
    }

    fn init_field() -> Base64VecU8 {
        let mut field = Base64VecU8::from(vec![0u8; FIELD_LEN]);

        return field;
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Game {
    pub field: Field,
    pub first_turn_time: Timestamp,
    pub last_turn_time: Timestamp,
}

impl Game {
    pub fn new(turn_time: Timestamp) -> Self {
        Self {
            field: Field::new(),
            first_turn_time: turn_time,
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
