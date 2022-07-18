//╦
//╩
//╣
//╠
//╔
//╚
//╗
//╝
//═
//║

mod cell;
mod field;
use cell::CellContent;
use field::Field;

fn main() {
    println!("Hello, world!");
    let trb = CellContent {content: '╠', top: 2, right: 2, bottom: 2, ..Default::default()};
    let trl = CellContent {content: '╩', top: 2, right: 2, left: 2, ..Default::default()};
    let tbl = CellContent {content: '╣', top: 2, bottom: 2, left: 2, ..Default::default()};
    let rbl = CellContent {content: '╦', right: 2, bottom: 2, left: 2, ..Default::default()};
    let tb = CellContent {content: '║', top: 2, bottom: 2, ..Default::default()};
    let rl = CellContent {content: '═', right: 2, left: 2, ..Default::default()};
    let tr = CellContent {content: '╚', top: 2, right: 2, ..Default::default()};
    let tl = CellContent {content: '╝', top: 2, left: 2, ..Default::default()};
    let rb = CellContent {content: '╔', right: 2, bottom: 2, ..Default::default()};
    let bl = CellContent {content: '╗', bottom: 2, left: 2, ..Default::default()};

    let empty = CellContent {content: ' ', ..Default::default()};
    // let all = CellContent {content: '+', top: 2, right: 2, bottom: 2, left:2 };

    // let celops = vec![trb,trl,tbl,rbl,empty];
    let celops = vec![trb,trl,tbl,rbl,tb,rl,tr,tl,rb,bl,empty];
    let mut field = Field::new(50,50, celops);
    while field.to_collapse().len() > 0 {
        field.print();
        println!("----------");
        let cell = field.get_random_cell_to_collapse().unwrap().to_owned();
        field.collapse_random_cell(&cell);
    }
    field.print();
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::cell::Cell;
    #[test]
    fn possible() {
        let trb = CellContent {content: '╠', top: 2, right: 2, bottom: 2, ..Default::default()};
        let trl = CellContent {content: '╩', top: 2, right: 2, left: 2, ..Default::default()};
        let tbl = CellContent {content: '╣', top: 2, bottom: 2, left: 2, ..Default::default()};
        let rbl = CellContent {content: '╦', right: 2, bottom: 2, left: 2, ..Default::default()};
        let empty = CellContent { ..Default::default()};
        let celops = vec![trb,trl,tbl,rbl,empty];

        let possible = Cell::possible(celops.as_slice(), None, None, Some(rbl), None);
        assert_eq!(2, possible.len());
        println!("{:?}", possible);
        assert!(possible.contains(&empty));
        assert!(possible.contains(&trl));
        assert!(!possible.contains(&trb));
        assert!(!possible.contains(&tbl));
        assert!(!possible.contains(&rbl));
        assert_eq!(celops.len(), 5);
        let possible = Cell::possible(celops.as_slice(), Some(trb), Some(empty), None, None);
        assert_eq!(1, possible.len());
    }

    #[test]
    fn collapse() {
        let trb = CellContent {content: '╠', top: 2, right: 2, bottom: 2, ..Default::default()};
        let mut cell: Cell = Default::default();
        cell.collapse(&[trb]);
        assert!(cell.content.is_some());
    }
}
