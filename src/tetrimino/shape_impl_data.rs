use crate::tetrimino::{TetriminoGenerator, Tetrimino};

pub struct TetriminoI;

impl TetriminoGenerator for TetriminoI {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                // -
                vec![vec![1, 1, 1, 1],
                     vec![0, 0, 0, 0],
                     vec![0, 0, 0, 0],
                     vec![0, 0, 0, 0]],
                // |
                vec![vec![0, 1, 0, 0],
                     vec![0, 1, 0, 0],
                     vec![0, 1, 0, 0],
                     vec![0, 1, 0, 0]]
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetriminoJ;

impl TetriminoGenerator for TetriminoJ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![vec![2, 2, 2, 0],
                     vec![2, 0, 0, 0],
                     vec![0, 0, 0, 0],
                     vec![0, 0, 0, 0]],
                // ⅂
                vec![vec![2, 2, 0, 0],
                     vec![0, 2, 0, 0],
                     vec![0, 2, 0, 0],
                     vec![0, 0, 0, 0]],
                vec![vec![0, 0, 2, 0],
                     vec![2, 2, 2, 0],
                     vec![0, 0, 0, 0],
                     vec![0, 0, 0, 0]],
                // L
                vec![vec![2, 0, 0, 0],
                     vec![2, 0, 0, 0],
                     vec![2, 2, 0, 0],
                     vec![0, 0, 0, 0]]
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetriminoL;

impl TetriminoGenerator for TetriminoL {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![vec![3, 3, 3, 0],
                     vec![0, 0, 3, 0],
                     vec![0, 0, 0, 0],
                     vec![0, 0, 0, 0]],
                // ⅃
                vec![vec![0, 3, 0, 0],
                     vec![0, 3, 0, 0],
                     vec![3, 3, 0, 0],
                     vec![0, 0, 0, 0]],
                vec![vec![3, 0, 0, 0],
                     vec![3, 3, 3, 0],
                     vec![0, 0, 0, 0],
                     vec![0, 0, 0, 0]],
                vec![vec![3, 3, 0, 0],
                     vec![3, 0, 0, 0],
                     vec![3, 0, 0, 0],
                     vec![0, 0, 0, 0]]
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetriminoO;

impl TetriminoGenerator for TetriminoO {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                // 口
                vec![vec![4, 4, 0, 0],
                     vec![4, 4, 0, 0],
                     vec![0, 0, 0, 0],
                     vec![0, 0, 0, 0]]
            ],
            x: 5,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetriminoS;

impl TetriminoGenerator for TetriminoS {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![vec![0, 5, 5, 0],
                     vec![5, 5, 0, 0],
                     vec![0, 0, 0, 0],
                     vec![0, 0, 0, 0]],
                vec![vec![0, 5, 0, 0],
                     vec![0, 5, 5, 0],
                     vec![0, 0, 5, 0],
                     vec![0, 0, 0, 0]]
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetriminoZ;

impl TetriminoGenerator for TetriminoZ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![vec![6, 6, 0, 0],
                     vec![0, 6, 6, 0],
                     vec![0, 0, 0, 0],
                     vec![0, 0, 0, 0]],
                vec![vec![0, 0, 6, 0],
                     vec![0, 6, 6, 0],
                     vec![0, 6, 0, 0],
                     vec![0, 0, 0, 0]]
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetriminoT;

impl TetriminoGenerator for TetriminoT {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![
                vec![vec![7, 7, 7, 0],
                     vec![0, 7, 0, 0],
                     vec![0, 0, 0, 0],
                     vec![0, 0, 0, 0]],
                vec![vec![0, 7, 0, 0],
                     vec![7, 7, 0, 0],
                     vec![0, 7, 0, 0],
                     vec![0, 0, 0, 0]],
                vec![vec![0, 7, 0, 0],
                     vec![7, 7, 7, 0],
                     vec![0, 0, 0, 0],
                     vec![0, 0, 0, 0]],
                vec![vec![0, 7, 0, 0],
                     vec![0, 7, 7, 0],
                     vec![0, 7, 0, 0],
                     vec![0, 0, 0, 0]]
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}