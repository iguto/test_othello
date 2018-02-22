use std::fmt;

pub struct Map {
    pub col_size: usize,
    pub row_size: usize,
    pub inner_map: Vec<Vec<Cell>>,
}

impl Map {
    pub fn new(row_size: usize, col_size: usize) -> Self {
        let mut vecs = vec![vec![Cell::Empty; col_size]; row_size];
        vecs[3][3] = Cell::Black;
        vecs[4][4] = Cell::Black;
        vecs[3][4] = Cell::White;
        vecs[4][3] = Cell::White;
        Map {
            row_size,
            col_size,
            inner_map: vecs,
        }
    }
}

#[derive(Clone)]
pub enum Cell {
    Empty,
    Black,
    White,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::Empty | Cell::Black | Cell::White => write!(f, " "),
        }
    }
}

impl Cell {
    pub fn put(&mut self, target: Cell) {
        *self = target.clone();
    }
}
