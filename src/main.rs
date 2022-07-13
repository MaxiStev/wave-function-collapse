use array2d;
use rand::{self, Rng};
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
    content: CellOp<'a>,
    top: Option<&'a Cell<'a>>,
    right: Option<&'a Cell<'a>>,
    bottom: Option<&'a Cell<'a>>,
    left: Option<&'a Cell<'a>>
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
    fn poss2<'a>(&self, allowed: &[&'a CellContent]) -> Vec<&'a CellContent> {
        let mut all: Vec<&CellContent> = allowed.to_vec();
        if let Some(neighbour) = self.top {
            if let Some(cell) = neighbour.content {
                all.retain(|c| c.top == cell.bottom);
            }
        }
        if let Some(neighbour) = self.right {
            if let Some(cell) = neighbour.content {
                all.retain(|c| c.right == cell.left);
            }
        }
        if let Some(neighbour) = self.bottom {
            if let Some(cell) = neighbour.content {
                all.retain(|c| c.bottom == cell.top);
            }
        }
        if let Some(neighbour) = self.left {
            if let Some(cell) = neighbour.content {
                all.retain(|c| c.left == cell.right);
            }
        }

        return all;
    }
}
impl<'a> Cell<'a> {
    fn collapse(&mut self, possible: &[&'a CellContent]) {
        if self.content.is_some() {
            return ;
        }
        let mut rng = rand::prelude::thread_rng();
        self.content = Some(possible.get(rng.gen_range(0..possible.len())).unwrap());
    }
}
impl Default for Cell<'_> {
    fn default() -> Self {
        Cell {content: None, top: None, right: None, bottom: None, left: None}
    }
}

struct Field<'a> {
    field: array2d::Array2D<Cell<'a>>,
    allowed_cells: Vec<CellContent>
}
impl Field<'_> {
    fn new(height: usize, width: usize, allowed_cells: Vec<CellContent>) -> Self {
        let mut field = array2d::Array2D::filled_with(Cell::default(), height, width);
        for row in 0..field.row_len() {
            for col in 0..field.column_len() {
                let mut top: Option<&Cell> = None;
                if let Some(r) = row.checked_sub(1) {
                    top = field.get(r, col);
                }
                let mut cell = field.get_mut(row, col).unwrap();
                cell.top = top;
            }
        }

        Self {field, allowed_cells}
    }
    fn print(&self) {
        for row in self.field.as_rows().iter() {
            for col in row.iter() {
                print!("{}", if let Some(content) = col.content {content.content} else {'X'});
            }
            println!();
        }
    }
    fn lowest_entropy (&self) -> Vec<&Cell>{
        let field_ref = self.allowed_cells.iter()
            .map(|c| c)
            .collect::<Vec<&CellContent>>();
        let mut to_collapse = self.field.elements_row_major_iter()
            .filter(|c| c.content.is_some())
            .collect::<Vec<&Cell>>();

        let lowest = to_collapse.iter()
            .map(|c| c.poss2(&field_ref).len())
            .reduce(|acc, item| {
                if acc < item {acc} else {item}
            }).expect("No item with entropy");
        to_collapse.retain(|cell| cell.poss2(&field_ref).len() == lowest);
        return to_collapse;
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
        let top = Cell {content: Some(&trb), ..Default::default()};
        let right = Cell {content: None, ..Default::default()};
        let bottom = Cell {content: None, ..Default::default()};
        let left = Cell {content: None, ..Default::default()};
        let celops = vec![&trb,&trl,&tbl,&rbl,&empty];

        let possible = Cell::possible(celops.as_slice(), &top.content, &right.content, &bottom.content, &left.content);
        assert_eq!(3, possible.len());
        assert_eq!(celops.len(), 5);
    }

    #[test]
    fn collapse() {
        let trb = CellContent {content: '╠', top: 2, right: 2, bottom: 2, ..Default::default()};
        let mut cell: Cell = Default::default();
        cell.collapse(&[&trb]);
        assert!(cell.content.is_some());
    }

    #[test]
    fn poss2() {
        let trb = CellContent {content: '╠', top: 2, right: 2, bottom: 2, ..Default::default()};
        let trl = CellContent {content: '╩', top: 2, right: 2, left: 2, ..Default::default()};
        let tbl = CellContent {content: '╣', top: 2, bottom: 2, left: 2, ..Default::default()};
        let rbl = CellContent {content: '╦', right: 2, bottom: 2, left: 2, ..Default::default()};
        let empty = CellContent {content: ' ', ..Default::default()};
        let top = Cell {content: Some(&trb), ..Default::default()};
        let right = Cell {content: None, ..Default::default()};
        let bottom = Cell {content: None, ..Default::default()};
        let celops = vec![&trb,&trl,&tbl,&rbl,&empty];

        let mut cell = Cell::default();
        cell.top = Some(&top);
        cell.right = Some(&right);
        cell.bottom = Some(&bottom);
        //No Cell Left
        let possible = cell.poss2(&celops);
        assert_eq!(3, possible.len());
        assert_eq!(celops.len(), 5);
    }
}
