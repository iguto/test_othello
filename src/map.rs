use std::fmt;

const BOARD_SIZE: usize = 8;
const DIRS: [(i64, i64); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

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

    pub fn get_cell(&self, coord: Coordinate) -> Option<Cell> {
        match self.inner_map.get(coord.0) {
            Some(column_data) => match column_data.get(coord.1) {
                Some(cell) => Some(*cell),
                None => return None,
            },
            None => return None,
        }
    }

    pub fn get_mut_cell(&mut self, coord: Coordinate) -> Option<&mut Cell> {
        match self.inner_map.get_mut(coord.0) {
            Some(column_data) => match column_data.get_mut(coord.1) {
                Some(cell) => Some(cell),
                None => return None,
            },
            None => return None,
        }
    }

    pub fn put_hand(&mut self, row: usize, column: usize, player: CellColors) {
        self.inner_map[row][column].color = player;
        let coord = Coordinate(row, column);
        self.flip_hands(coord, player);
    }

    fn flip_hands(&mut self, coord: Coordinate, player: CellColors) {
        for dir in DIRS.iter() {
            self.flip_hands_dir(coord, player, *dir);
        }
    }

    fn flip_hands_dir(&mut self, coord: Coordinate, player: CellColors, dir: (i64, i64)) {
        let mut coord: Coordinate = coord.clone();
        if !self.is_reversible_dir(coord, player, dir) {
            return;
        }
        loop {
            coord = match coord.next(dir) {
                Some(c) => c,
                None => break,
            };
            let cell: &mut Cell = match self.get_mut_cell(coord) {
                Some(cell) => cell,
                None => panic!("out of bound error"),
            };
            match player {
                CellColors::Black => match cell.color {
                    CellColors::White => cell.color = CellColors::Black,
                    _ => break,
                },
                CellColors::White => match cell.color {
                    CellColors::Black => cell.color = CellColors::White,
                    _ => break,
                },
                _ => unreachable!(),
            }
        }
    }

    pub fn is_reversible(&self, coord: Coordinate, player: CellColors) -> bool {
        for dir in DIRS.iter() {
            if self.is_reversible_dir(coord, player, *dir) {
                return true;
            }
        }
        false
    }

    fn is_reversible_dir(&self, coord: Coordinate, player: CellColors, dir: (i64, i64)) -> bool {
        let mut coord = coord.clone();
        let mut reversible_count = 0;

        loop {
            coord = match coord.next(dir) {
                Some(coord) => coord,
                None => return false,
            };

            let cell_color = match self.get_cell(coord) {
                Some(cell) => cell,
                None => panic!("out of bound error"),
            }.color;
            match player {
                CellColors::Black => match cell_color {
                    CellColors::White => reversible_count += 1,
                    CellColors::Black if reversible_count > 0 => return true,
                    _ => return false,
                },
                CellColors::White => match cell_color {
                    CellColors::Black => reversible_count += 1,
                    CellColors::White if reversible_count > 0 => return true,
                    _ => return false,
                },
                _ => return false,
            };
        }
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

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub column: usize,
    pub row: usize,
    pub color: CellColors,
}

impl fmt::Display for CellColors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CellColors::Black => write!(f, "black"),
            CellColors::White => write!(f, "white"),
            CellColors::Empty => write!(f, "empty"),
        }
    }
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

    pub fn hand(&mut self, player: CellColors) {
        println!("flip called cell:{:?}", self);
        self.color = player;
    }
}
