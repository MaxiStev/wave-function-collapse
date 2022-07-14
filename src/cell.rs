use rand::Rng;
pub type CellOp<'a> = Option<&'a CellContent>;
#[derive(Debug, Clone)]
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

#[derive(Clone)]
pub struct Cell<'a> {
    pub content: CellOp<'a>,
    //position row, col
}
impl Cell<'_> {
    pub fn possible_tuple<'a>(allowed: &[&'a CellContent], surrounding: (CellOp, CellOp, CellOp, CellOp)) -> Vec<&'a CellContent> {
        Cell::possible(allowed, surrounding.0, surrounding.1, surrounding.2, surrounding.3)
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
}
impl<'a> Cell<'a> {
    pub fn collapse(&mut self, possible: &[&'a CellContent]) {
        if self.content.is_some() {
            return ;
        }
        let mut rng = rand::prelude::thread_rng();
        self.content = Some(possible.get(rng.gen_range(0..possible.len())).unwrap());
    }
}
impl Default for Cell<'_> {
    fn default() -> Self {
        Cell {content: None}
    }
}
