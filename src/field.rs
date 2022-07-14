use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Field {
    pub field: Vec<Vec<i8>>,
    pub playing_field: Base64VecU8,  // 1 - opened, 0 - closed
    pub incidence_matrix: Base64VecU8,
}

impl Field {
    pub fn new() -> Self {
        let mut field = vec![vec![0i8; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize];
        let mut incidence_matrix = Base64VecU8::from(vec![0u8; FIELD_LEN]);

        Field::fill_matrices(&mut field, &mut incidence_matrix);

        Self {
            field: field,
            playing_field: Base64VecU8::from(vec![0u8; FIELD_LEN]),
            incidence_matrix: incidence_matrix,
        }
    }

    fn fill_matrices(field: &mut Vec<Vec<i8>>, incidence_matrix: &mut Base64VecU8) {
        /* For incidence matrix
            let index = y * FIELD_WIDTH as usize + x;
            let byte_index = index / 8;
            let bit_index = index & 7;
        */

        let mut mines_coordinates = Field::fill_mines(field);

        Field::fill_with_values(field);

        Field::fill_incidence_matrix(field, incidence_matrix);
    }

    fn fill_mines(field: &mut Vec<Vec<i8>>) -> HashSet<(usize, usize)> {
        let mut mines_coordinates: HashSet<(usize, usize)> = HashSet::new();

        for _i in 0..AMOUNT_OF_MINES {
            let x = get_random_number(AMOUNT_OF_MINES) as usize;
            let y = get_random_number(AMOUNT_OF_MINES) as usize;

            let cell = field[y][x];
            if cell != -1 {
                field[y][x] = -1;

                mines_coordinates.insert((x, y));
            }
        }

        return mines_coordinates;
    }

    fn fill_with_values(field: &mut Vec<Vec<i8>>) {
        for i in 0..FIELD_HEIGHT as usize {
            for j in 0..FIELD_WIDTH as usize {
                if field[i][j] == 0 {
                    field[i][j] = Field::mines_around(field, i as i8, j as i8);
                }
            }
        }
    }

    fn mines_around(field: &mut Vec<Vec<i8>>, r: i8, c: i8) -> i8 {
        let mut mines_around: i8 = 0;

        for k in -1..2 {
            for l in -1..2 {
                let x: i8 = k + r;
                let y: i8 = l + c;

                if x < 0 || y < 0 || x >= FIELD_WIDTH as i8 || y >= FIELD_HEIGHT as i8 {
                    continue;
                }

                if field[x as usize][y as usize] < 0 {
                    mines_around += 1;
                }
            }
        }

        return mines_around;
    }

    fn fill_incidence_matrix(field: &Vec<Vec<i8>>, incidence_matrix: &mut Base64VecU8) {
        // ...
    }
}