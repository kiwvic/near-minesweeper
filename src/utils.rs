use crate::*;

pub(crate) fn hash_from_random_seed() -> Vec<u8> {
    let random_seed = [*env::random_seed().get(0).unwrap()];

    env::keccak256(&random_seed)
}

pub(crate) fn set_bit(arr: &mut Base64VecU8, i: u8, j: u8) {
    let index = (i * FIELD_WIDTH + j) as usize;
    let byte_index = index / 8;
    let bit_index = index & 7; // byte_index % 8;

    arr.0[byte_index] |= 1u8 << bit_index;
    arr.0[byte_index] ^= 1u8 << bit_index;
}

pub(crate) fn is_bit_set(arr: &Base64VecU8, i: u8, j: u8) -> bool {
    let index = (i * FIELD_WIDTH + j) as usize;
    let byte_index = index / 8;
    let bit_index = index & 7; // byte_index % 8;
    
    ((arr.0[byte_index] >> bit_index) & 1) != 0
}