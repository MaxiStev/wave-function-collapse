use std::fmt::Display;

use rand::Rng;
pub mod default;
pub type CellOp<T> = Option<CellContent<T>>;
#[derive(Debug, Clone, Copy)]
pub struct CellContent<Content> {
    pub top: u8,
    pub right: u8,
    pub bottom: u8,
    pub left: u8,
    pub content: Content,
}

impl<T: Display> Display for CellContent<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl<T: Default> Default for CellContent<T> {
    fn default() -> Self {
        CellContent {
            top: 0,
            right: 0,
            bottom: 0,
            left: 0,
            content: T::default(),
        }
    }
}

impl<T: PartialEq> PartialEq for CellContent<T> {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
            && self.top == other.top
            && self.right == other.right
            && self.bottom == other.bottom
            && self.left == other.left
    }
}

#[derive(Clone)]
pub struct Cell<T: Clone> {
    pub content: CellOp<T>,
    pub row: usize,
    pub col: usize,
    pub possible: Vec<CellContent<T>>,
    //position row, col
}
impl<T: PartialEq + Clone> PartialEq for Cell<T> {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content && self.row == other.row && self.col == other.col
    }
}

impl<T: Clone> Cell<T> {
    pub fn entropy(&self) -> usize {
        self.possible.len()
    }
    pub fn update_top(&mut self, cont: CellContent<T>) {
        self.possible.retain(|c| c.top == cont.bottom)
    }
    pub fn update_right(&mut self, cont: CellContent<T>) {
        self.possible.retain(|c| c.right == cont.left)
    }
    pub fn update_bottom(&mut self, cont: CellContent<T>) {
        self.possible.retain(|c| c.bottom == cont.top)
    }
    pub fn update_left(&mut self, cont: CellContent<T>) {
        self.possible.retain(|c| c.left == cont.right)
    }
    pub fn collapse(&mut self, rng: &mut rand::prelude::ThreadRng) {
        if self.content.is_some() {
            panic!();
        }
        self.content = Some(
            self.possible
                .get(rng.gen_range(0..self.possible.len()))
                .unwrap()
                .to_owned(),
        );
    }
    pub fn set_possible(&mut self, possible: Vec<CellContent<T>>) {
        self.possible = possible
            .iter()
            .map(|c| c.to_owned())
            .collect::<Vec<CellContent<T>>>();
    }
}
impl<T: Copy> Default for Cell<T> {
    fn default() -> Self {
        Cell {
            content: None,
            row: 0,
            col: 0,
            possible: vec![],
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    pub struct DefaultCellContent {
        pub trb: CellContent<char>,
        pub trl: CellContent<char>,
        pub tbl: CellContent<char>,
        pub rbl: CellContent<char>,
        pub empty: CellContent<char>,
    }
    impl DefaultCellContent {
        pub fn new() -> DefaultCellContent {
            let trb = CellContent {
                content: '???',
                top: 2,
                right: 2,
                bottom: 2,
                ..Default::default()
            };
            let trl = CellContent {
                content: '???',
                top: 2,
                right: 2,
                left: 2,
                ..Default::default()
            };
            let tbl = CellContent {
                content: '???',
                top: 2,
                bottom: 2,
                left: 2,
                ..Default::default()
            };
            let rbl = CellContent {
                content: '???',
                right: 2,
                bottom: 2,
                left: 2,
                ..Default::default()
            };
            let empty = CellContent {
                ..Default::default()
            };
            Self {
                trb,
                trl,
                tbl,
                rbl,
                empty,
            }
        }
        pub fn vec(&self) -> Vec<CellContent<char>> {
            vec![self.trb, self.trl, self.tbl, self.rbl, self.empty]
        }
    }

    #[test]
    fn possible() {
        let def = DefaultCellContent::new();
        let mut cell = Cell::default();
        let celops = def.vec();
        cell.set_possible(celops.to_owned());

        cell.update_bottom(def.rbl);
        let possible = cell.possible.to_owned();
        assert_eq!(2, possible.len());
        println!("{:?}", possible);
        assert!(possible.contains(&def.empty));
        assert!(possible.contains(&def.trl));
        assert!(!possible.contains(&def.trb));
        assert!(!possible.contains(&def.tbl));
        assert!(!possible.contains(&def.rbl));
        assert_eq!(celops.len(), 5);
        cell.set_possible(celops.to_owned());
        cell.update_top(def.trb);
        cell.update_right(def.empty);
        assert_eq!(1, cell.possible.len());
    }

    #[test]
    fn collapse() {
        let trb = CellContent {
            content: '???',
            top: 2,
            right: 2,
            bottom: 2,
            ..Default::default()
        };
        let mut rng = rand::prelude::thread_rng();
        let mut cell: Cell<char> = Default::default();
        cell.set_possible(vec![trb]);
        cell.collapse(&mut rng);
        assert!(cell.content.is_some());
    }
    #[test]
    fn update_right() {
        let def = DefaultCellContent::new();
        let mut cell = Cell::default();
        cell.set_possible(def.vec());
        cell.update_right(def.trb);
        assert_eq!(cell.possible.len(), 2);
        let possible = cell.possible.to_owned();
        assert!(possible.contains(&def.empty));
        assert!(possible.contains(&def.tbl));
    }
}
