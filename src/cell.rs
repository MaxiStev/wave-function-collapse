use rand::Rng;
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
            self.left == other.left
    }
}

#[derive(Clone)]
pub struct Cell {
    pub content: CellOp,
    pub row: usize,
    pub col: usize,
    pub possible: Vec<CellContent>,
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
    pub fn entropy(&self) -> usize {
        self.possible.len()
    }
    pub fn update_top(&mut self, cont: CellContent) {
        self.possible.retain(|c| c.top == cont.bottom)
    }
    pub fn update_right(&mut self, cont: CellContent) {
        self.possible.retain(|c| c.right == cont.left)
    }
    pub fn update_bottom(&mut self, cont: CellContent) {
        self.possible.retain(|c| c.bottom == cont.top)
    }
    pub fn update_left(&mut self, cont: CellContent) {
        self.possible.retain(|c| c.left == cont.right)
    }
    pub fn collapse(&mut self) {
        if self.content.is_some() {
            panic!();
        }
        let mut rng = rand::prelude::thread_rng();
        self.content = Some(self.possible.get(rng.gen_range(0..self.possible.len())).unwrap().to_owned());
    }
    pub fn set_possible(&mut self, possible: Vec<CellContent>) {
        self.possible = possible.iter().map(|c| c.to_owned()).collect::<Vec<CellContent>>();
    }
}
impl Drop for Cell {
    fn drop(&mut self) {
        println!("cell dropped");
    }
}
impl Default for Cell {
    fn default() -> Self {
        Cell {content: None, row: 0, col: 0, possible: vec![]}
    }
}
