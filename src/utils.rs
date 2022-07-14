use crate::*;

pub(crate) fn get_random_number(divider: u8) -> u8 {
    *env::random_seed().get(0).unwrap() / divider as u8
}
