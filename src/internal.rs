use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Field {
    pub field: Vec<Vec<i8>>,
    pub incidence_matrix: Base64VecU8,
}

impl Field {
    pub fn new() -> Self {
        let mut field = vec![vec![0i8; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize];
        let mut incidence_matrix = Base64VecU8::from(vec![0u8; FIELD_LEN]);

        Field::fill_matrices(&mut field, &mut incidence_matrix);

        Self {
            field: field,
            incidence_matrix: incidence_matrix,
        }
    }

    fn fill_matrices(field: &mut Vec<Vec<i8>>, incidence_matrix: &mut Base64VecU8) {
        let mut mines_coordinates: HashSet<(usize, usize)> = HashSet::new();

        for _i in 0..AMOUNT_OF_MINES {
            let x: usize = get_random_number(AMOUNT_OF_MINES) as usize;
            let y: usize = get_random_number(AMOUNT_OF_MINES) as usize;

            // For incidence matrix
            let index: usize = y * FIELD_WIDTH as usize + x;
            let byte_index = index / 8;
            let bit_index = index & 7;

            let cell = field[y][x];
            if cell != -1 {
                field[y][x] = -1;

                mines_coordinates.insert((x, y));
            }
        }
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
