mod shape_impl_data;


extern crate rand;

/*
 * shape of the tetrimino
 * for example
 * ```
 * vec![vec![1, 1, 0, 0],
 *      vec![1, 1, 0, 0],
 *      vec![0, 0, 0, 0],
 *      vec![0, 0, 0, 0]]
 * ```
 * is  a cube block
 */
type Piece = Vec<Vec<u8>>;

#[derive(Clone)]
pub struct Tetrimino {
    pub states: Vec<Piece>,
    pub x: isize,
    pub y: usize,
    pub current_state: u8,
}

trait TetriminoGenerator {
    fn new() -> Tetrimino;
}

pub fn create_new_tetrimino() -> Tetrimino {
    static mut PREV: u8 = 7;
    let mut rand_nb = rand::random::<u8>() % 7;
    if unsafe { PREV } == rand_nb {
        rand_nb = rand::random::<u8>() % 7;
    }
    unsafe { PREV = rand_nb; }
    match rand_nb {
        0 => shape_impl_data::TetriminoI::new(),
        1 => shape_impl_data::TetriminoJ::new(),
        2 => shape_impl_data::TetriminoL::new(),
        3 => shape_impl_data::TetriminoO::new(),
        4 => shape_impl_data::TetriminoS::new(),
        5 => shape_impl_data::TetriminoZ::new(),
        6 => shape_impl_data::TetriminoT::new(),
        _ => unreachable!(),
    }
}

impl Tetrimino {
    pub fn test_current_position(&self, game_map: &[Vec<u8>]) -> bool {
        self.test_position(game_map, self.current_state as usize,
                           self.x, self.y)
    }

    pub fn test_position(&self, game_map: &[Vec<u8>],
                     tmp_state: usize, x: isize, y: usize) -> bool {
        for decal_y in 0..4 {
            for decal_x in 0..4 {
                let x = x + decal_x;
                if self.states[tmp_state][decal_y][decal_x as usize] != 0
                    &&
                    (y + decal_y >= game_map.len() ||
                        x < 0 ||
                        x as usize >= game_map[y + decal_y].len() ||
                        game_map[y + decal_y][x as usize] != 0) {
                    return false;
                }
            }
        }
        true
    }

    pub fn rotate(&mut self, game_map: &[Vec<u8>]) {
        let mut tmp_state = self.current_state + 1;
        if tmp_state as usize >= self.states.len() {
            tmp_state = 0;
        }
        let x_pos = [0, -1, 1, -2, 2, -3];
        for x in x_pos.iter() {
            if self.test_position(game_map, tmp_state as usize,
                                  self.x + x, self.y) == true {
                self.current_state = tmp_state;
                self.x += *x;
                break;
            }
        }
    }

    pub fn change_position(&mut self, game_map: &[Vec<u8>], new_x: isize, new_y: usize) -> bool {
        if self.test_position(game_map, self.current_state as usize,
                              new_x, new_y) == true {
            self.x = new_x as isize;
            self.y = new_y;
            true
        } else {
            false
        }
    }
}