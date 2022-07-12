//╠
//╩
//╣
//╦
//═
//╚



struct CellOp {
    top: u8,
    right: u8,
    bottom: u8,
    left: u8,
    content: char
}
impl Default for CellOp {
    fn default() -> Self {
        CellOp {top: 0, right: 0, bottom: 0, left: 0, content: ' '}
    }
}
fn main() {
    println!("Hello, world!");
    let trb = CellOp {content: '╠', top: 2, right: 2, bottom: 2, ..Default::default()};
    let trl = CellOp {content: '╩', top: 2, right: 2, left: 2, ..Default::default()};
    let trl = CellOp {content: '╣', top: 2, bottom: 2, left: 2, ..Default::default()};
    let rbl = CellOp {content: '╦', right: 2, bottom: 2, left: 2, ..Default::default()};

    let mut cell1 = Cell::default();
    let mut cell2 = Cell::default();
    cell1.right = Some(&cell2);
    cell2.left = Some(&cell1);
}
struct Cell<'a> {
    top: Option<&'a Cell<'a>>,
    right: Option<&'a Cell<'a>>,
    bottom: Option<&'a Cell<'a>>,
    left: Option<&'a Cell<'a>>,
    content: Option<CellOp>
}
impl Default for Cell<'_> {
    fn default() -> Self {
        Cell {top: None, right: None, bottom: None, left: None, content: None}
    }
}
