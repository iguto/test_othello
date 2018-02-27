use std::fmt;

const BOARD_SIZE: usize = 8;
type Board = [[Cell; BOARD_SIZE]; BOARD_SIZE];

#[derive(Clone, Copy, Debug)]
pub struct Coordinate(pub usize, pub usize);

pub struct Map {
    pub col_size: usize,
    pub row_size: usize,
    pub inner_map: Board,
}

impl Coordinate {
    pub fn next(&self, other: (i64, i64)) -> Option<Coordinate> {
        let row = other.0 + (self.0 as i64);
        let column = other.1 + (self.1 as i64);
        if row < 0 || column < 0 || row >= BOARD_SIZE as i64 || column >= BOARD_SIZE as i64 {
            return None;
        }
        Some(Coordinate(row as usize, column as usize))
    }
}

impl Map {
    pub fn new(row_size: usize, col_size: usize) -> Self {
        let board = Self::setup_board();
        Map {
            row_size,
            col_size,
            inner_map: board,
        }
    }

    pub fn put_hand(&mut self, row: usize, column: usize, player: CellColors) {
        self.inner_map[row][column].color = player;
    }

    fn setup_board() -> Board {
        let cell = Cell {
            row: 0,
            column: 0,
            color: CellColors::Empty,
        };
        let mut board = [[cell.clone(); BOARD_SIZE]; BOARD_SIZE];

        // set position to every cells
        for (row, column_cells) in board.iter_mut().enumerate() {
            let row = row.clone();
            for (column, cell) in column_cells.iter_mut().enumerate() {
                cell.set_position(row, column);
            }
        }
        Self::put_hands_on_init(&mut board)
    }

    fn put_hands_on_init(board: &mut Board) -> Board {
        board[3][3].color = CellColors::Black;
        board[4][4].color = CellColors::Black;
        board[3][4].color = CellColors::White;
        board[4][3].color = CellColors::White;
        *board
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CellColors {
    Empty,
    White,
    Black,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub column: usize,
    pub row: usize,
    pub color: CellColors,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.color {
            CellColors::Empty | CellColors::Black | CellColors::White => write!(f, " "),
        }
    }
}

impl Cell {
    fn set_position(&mut self, row: usize, column: usize) {
        self.row = row;
        self.column = column;
    }
}
