use array2d;
//╠
//╩
//╣
//╦
//═
//╚

mod cell;
mod field;
use cell::{Cell, CellContent, CellOp};
use field::Field;

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

        let possible = Cell::possible(celops.as_slice(), top.content, right.content, bottom.content, left.content);
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
}
