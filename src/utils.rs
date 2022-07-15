use crate::*;

pub(crate) fn get_random_number(divider: u8) -> u8 {
    *env::random_seed().get(0).unwrap() / divider as u8
}

pub(crate) fn set_bit(arr: Base64VecU8, i: u8, j: u8) {
    /* For incidence matrix
        let index = y * FIELD_WIDTH as usize + x;
        let byte_index = index / 8;
        let bit_index = index & 7;
    */
}

pub(crate) fn get_bit(arr: Base64VecU8, i: u8, j: u8) {
    // ...
}