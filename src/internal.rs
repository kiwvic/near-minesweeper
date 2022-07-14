use crate::*;

#[derive(BorshStorageKey, BorshSerialize)]
pub struct PlayerStats {
    pub wins: u32,
    pub losses: u32,

    pub fastest_game: u32,
    pub longest_game: u32,
}
