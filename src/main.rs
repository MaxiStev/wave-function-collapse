use array2d;
//╠
//╩
//╣
//╦
//═
//╚


type CellOp<'a> = Option<&'a CellContent>;
#[derive(Debug, Clone)]
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
    let tbl = CellContent {content: '╣', top: 2, bottom: 2, left: 2, ..Default::default()};
    let rbl = CellContent {content: '╦', right: 2, bottom: 2, left: 2, ..Default::default()};
    let empty = CellContent {content: ' ', ..Default::default()};

    let celops = vec![trb,trl,tbl,rbl,empty];
    let field = Field::new(4,4, celops);
    field.print();
}
#[derive(Clone)]
struct Cell<'a> {
    content: CellOp<'a>
}
impl Cell<'_> {
    fn possible<'a>(allowed: &[&'a CellContent], top: &CellOp, right: &CellOp, bottom: &CellOp, left: &CellOp) -> Vec<&'a CellContent> {
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
impl Default for Cell<'_> {
    fn default() -> Self {
        Cell {content: None}
    }
}

struct Field<'a> {
    field: array2d::Array2D<Cell<'a>>,
    allowed_cells: Vec<CellContent>
}
impl Field<'_> {
    fn new(height: usize, width: usize, allowed_cells: Vec<CellContent>) -> Self {
        Self {field: array2d::Array2D::filled_with(Cell::default(), height, width), allowed_cells}
    }
    fn print(&self) {
        for row in self.field.as_rows().iter() {
            for col in row.iter() {
                print!("{}", if let Some(content) = col.content {content.content} else {'X'});
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn possible() {
        let trb = CellContent {content: '╠', top: 2, right: 2, bottom: 2, ..Default::default()};
        let trl = CellContent {content: '╩', top: 2, right: 2, left: 2, ..Default::default()};
        let tbl = CellContent {content: '╣', top: 2, bottom: 2, left: 2, ..Default::default()};
        let rbl = CellContent {content: '╦', right: 2, bottom: 2, left: 2, ..Default::default()};
        let empty = CellContent {content: ' ', ..Default::default()};
        let top = Cell {content: Some(&trb)};
        let right = Cell {content: None};
        let bottom = Cell {content: None};
        let left = Cell {content: None};
        let celops = vec![&trb,&trl,&tbl,&rbl,&empty];

        let possible = Cell::possible(celops.as_slice(), &top.content, &right.content, &bottom.content, &left.content);
        assert_eq!(3, possible.len());
        assert_eq!(celops.len(), 5);
    }
}
