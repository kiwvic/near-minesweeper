use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LazyOption, Vector};
use near_sdk::json_types::Base64VecU8;
use near_sdk::{
    env, near_bindgen, BorshStorageKey, 
    AccountId, Timestamp, PanicOnDefault
};
use std::collections::HashSet;

mod internal;
mod field;
mod utils;
mod game;

use crate::utils::*;
use crate::field::*;
use crate::game::*;
use crate::internal::*;

const FIELD_WIDTH: u8 = 16;
const FIELD_HEIGHT: u8 = 16;
const FIELD_LEN: usize = ((FIELD_WIDTH / 8) * FIELD_HEIGHT) as usize;
const AMOUNT_OF_MINES: u8 = 16;

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

    pub fn start_game(&mut self) {
        assert_eq!(
            self.current_games.contains_key(&env::current_account_id()), 
            true,
            "You are already in game!"
        );

        self.current_games.insert(&env::current_account_id(), &Game::new());
    }
}
