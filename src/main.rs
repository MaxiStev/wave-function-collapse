//╠
//╩
//╣
//╦
//═
//╚


type CellOp = Option<CellContent>;
struct CellContent {
    top: u8,
    right: u8,
    bottom: u8,
    left: u8,
    content: char
}
impl Default for CellContent {
    fn default() -> Self {
        CellContent {top: 0, right: 0, bottom: 0, left: 0, content: ' '}
    }
}
fn main() {
    println!("Hello, world!");
    let trb = CellContent {content: '╠', top: 2, right: 2, bottom: 2, ..Default::default()};
    let trl = CellContent {content: '╩', top: 2, right: 2, left: 2, ..Default::default()};
    let trl = CellContent {content: '╣', top: 2, bottom: 2, left: 2, ..Default::default()};
    let rbl = CellContent {content: '╦', right: 2, bottom: 2, left: 2, ..Default::default()};

    let mut cell1 = Cell::default();
    let mut cell2 = Cell::default();
}
struct Cell {
    content: CellOp
}
impl Cell {
    fn possible(allowed: Vec<CellContent>, top: CellOp, right: CellOp, bottom: CellOp, left: CellOp) -> Vec<CellContent> {

    }
}
impl Default for Cell {
    fn default() -> Self {
        Cell {content: None}
    }
}

struct Field {
    field: Vec<Vec<Cell>>,
    allowed_cells: Vec<CellContent>
}
