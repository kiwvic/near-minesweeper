use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LazyOption, Vector};
use near_sdk::json_types::Base64VecU8;
use near_sdk::{
    env, near_bindgen, BorshStorageKey, 
    AccountId, Timestamp, PanicOnDefault
};

mod internal;

use crate::internal::*;

const FIELD_WIDTH: usize = 16;
const FIELD_HEIGHT: usize = 16;
const FIELD_LEN: usize = (FIELD_WIDTH / 8) * FIELD_HEIGHT;
const AMOUNT_OF_MINES: usize = 16;

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    CurrentGames,
    PlayerStats,
    TopPlayers
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Minesweeper {
    current_games: LookupMap<AccountId, Game>,
    players_stats: LookupMap<AccountId, PlayerStats>,
    top_players: Vector<AccountId>,
}

#[near_bindgen]
impl Minesweeper {
    #[init]
    pub fn new() -> Self {
        Self {
            current_games: LookupMap::new(StorageKeys::CurrentGames),
            players_stats: LookupMap::new(StorageKeys::PlayerStats),
            top_players: Vector::new(StorageKeys::TopPlayers),
        }
    }

    pub fn start_game() {
        // TODO check if already in game
    }
}
