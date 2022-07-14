use rand::Rng;
use crate::field::Field;
pub type CellOp = Option<CellContent>;
#[derive(Debug, Clone, Copy)]
pub struct CellContent {
    pub top: u8,
    pub right: u8,
    pub bottom: u8,
    pub left: u8,
    pub content: char
}

impl Default for CellContent {
    fn default() -> Self {
        CellContent {top: 0, right: 0, bottom: 0, left: 0, content: ' '}
    }
}

impl PartialEq for CellContent {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content &&
            self.top == other.top &&
            self.right == other.right &&
            self.bottom == other.bottom &&
            self.left == other.bottom
    }
}

#[derive(Clone)]
pub struct Cell {
    pub content: CellOp,
    pub row: usize,
    pub col: usize
    //position row, col
}
impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content &&
            self.row == other.row &&
            self.col == other.col
    }
}
impl Cell {
    pub fn possible_tuple<'a>(allowed: &[&'a CellContent], surrounding: (CellOp, CellOp, CellOp, CellOp)) -> Vec<&'a CellContent> {
        Cell::possible(allowed, surrounding.0, surrounding.1, surrounding.2, surrounding.3)
    }

    pub fn possible_self<'a>(&self, field: &'a Field) -> Vec<&'a CellContent> {
        Cell::possible_tuple(&field.get_allowed_cells(), field.surrounding(self.row, self.col))
    }

    pub fn possible<'a>(allowed: &[&'a CellContent], top: CellOp, right: CellOp, bottom: CellOp, left: CellOp) -> Vec<&'a CellContent> {
        let mut all: Vec<&CellContent> = allowed.to_vec();
        if let Some(cell) = top {
            all.retain(|c| c.top == cell.bottom);
        }
        if let Some(cell) = right {
            all.retain(|c| c.right == cell.left);
        }
        if let Some(cell) = bottom {
            all.retain(|c| c.bottom == cell.top);
        }
        if let Some(cell) = left {
            all.retain(|c| c.left == cell.right);
        }
        all
    }
    pub fn collapse(&mut self, possible: &[CellContent]) {
        if self.content.is_some() {
            return ;
        }
        let mut rng = rand::prelude::thread_rng();
        self.content = Some(possible.get(rng.gen_range(0..possible.len())).unwrap().to_owned());
    }
}
impl Default for Cell {
    fn default() -> Self {
        Cell {content: None, row: 0, col: 0}
    }
}
