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
