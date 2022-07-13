use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LazyOption, Vector};
use near_sdk::json_types::Base64VecU8;
use near_sdk::{
    env, near_bindgen, BorshStorageKey, 
    AccountId, Timestamp, PanicOnDefault
};


const FIELD_WIDTH: usize = 16;
const FIELD_HEIGHT: usize = 16;
const FIELD_LEN: usize = (FIELD_WIDTH / 8) * FIELD_HEIGHT;


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


#[derive(BorshSerialize, BorshDeserialize)]
pub struct TopPlayers {
    pub list: Vector<AccountId>,
}


#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    CurrentGames,
    PlayerStats,
    TopPlayers
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Minesweeper {
    current_games: LookupMap<AccountId, GameInfo>,
    players_stats: LookupMap<AccountId, PlayerStats>,
    top_players: LazyOption<TopPlayers>,
}
