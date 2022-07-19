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

fn default_chars() -> Vec<CellContent> {
    vec![
        CellContent {content: ' ', ..Default::default()},
        CellContent {content: '╠', top: 2, right: 2, bottom: 2, ..Default::default()},
        CellContent {content: '╩', top: 2, right: 2, left: 2, ..Default::default()},
        CellContent {content: '╣', top: 2, bottom: 2, left: 2, ..Default::default()},
        CellContent {content: '╦', right: 2, bottom: 2, left: 2, ..Default::default()},
        CellContent {content: '║', top: 2, bottom: 2, ..Default::default()},
        CellContent {content: '═', right: 2, left: 2, ..Default::default()},
        CellContent {content: '╚', top: 2, right: 2, ..Default::default()},
        CellContent {content: '╝', top: 2, left: 2, ..Default::default()},
        CellContent {content: '╔', right: 2, bottom: 2, ..Default::default()},
        CellContent {content: '╗', bottom: 2, left: 2, ..Default::default()},
        CellContent {content: '╬', top: 2, right: 2, bottom: 2, left:2 },
        CellContent {content: '│', top: 1, bottom: 1, ..Default::default()},
        CellContent {content: '┤', top: 1, bottom: 1, left: 1, ..Default::default()},
        CellContent {content: '╡', top: 1, bottom: 1, left: 2, ..Default::default()},
        CellContent {content: '╢', top: 2, bottom: 2, left: 1, ..Default::default()},
        CellContent {content: '╖', bottom: 2, left: 1, ..Default::default()},
        CellContent {content: '╕', bottom: 1, left: 2, ..Default::default()},
        CellContent {content: '╜', top: 2, left: 1, ..Default::default()},
        CellContent {content: '╛', top: 1, left: 2, ..Default::default()},
        CellContent {content: '┐', bottom: 1, left: 1, ..Default::default()},
        CellContent {content: '└', top: 1, right: 1, ..Default::default()},
        CellContent {content: '┴', top: 1, right:1, left: 1, ..Default::default()},
        CellContent {content: '┬', right: 1, bottom: 1, left: 1,..Default::default()},
        CellContent {content: '├', top: 1, right: 1, bottom: 1, ..Default::default()},
        CellContent {content: '─', right: 1, left: 1, ..Default::default()},
        CellContent {content: '┼', top:1 , right:1, bottom:1, left: 1, ..Default::default()},
        CellContent {content: '╞', top: 1, right: 2, bottom:1, ..Default::default()},
        CellContent {content: '╟', top: 2, right: 1, bottom: 2, ..Default::default()},
        CellContent {content: '╧', top: 1, right: 2, left: 2, ..Default::default()},
        CellContent {content: '╨', top: 2, right: 1, left: 1, ..Default::default()},
        CellContent {content: '╤', right: 2, bottom: 1, left: 2, ..Default::default()},
        CellContent {content: '╥', right: 1, bottom: 2, left: 1, ..Default::default()},
        CellContent {content: '╙', top: 2, right: 1, ..Default::default()},
        CellContent {content: '╘', top: 1, right: 2, ..Default::default()},
        CellContent {content: '╒', right: 2, bottom: 1, ..Default::default()},
        CellContent {content: '╓', right:1 , bottom: 2, ..Default::default()},
        CellContent {content: '╫', top: 2, right: 1, bottom: 2, left: 1, ..Default::default()},
        CellContent {content: '╪', top: 1, right: 2, bottom: 1, left: 2, ..Default::default()},
        CellContent {content: '┘', top: 1, left: 1, ..Default::default()},
        CellContent {content: '┌', right: 1, bottom: 1, ..Default::default()},
        // CellContent {content: '', ..Default::default()},

    ]
}

fn main() {
    println!("Hello, world!");


    let celops = default_chars();
    let mut field = Field::new(10,10, celops);
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
        let mut cell = Cell::default();
        cell.set_possible(celops.to_owned());

        cell.update_bottom(rbl);
        let possible = cell.possible.to_owned();
        assert_eq!(2, possible.len());
        println!("{:?}", possible);
        assert!(possible.contains(&empty));
        assert!(possible.contains(&trl));
        assert!(!possible.contains(&trb));
        assert!(!possible.contains(&tbl));
        assert!(!possible.contains(&rbl));
        assert_eq!(celops.len(), 5);
        cell.set_possible(celops.to_owned());
        cell.update_top(trb);
        cell.update_right(empty);
        assert_eq!(1, cell.possible.len());
    }

    #[test]
    fn collapse() {
        let trb = CellContent {content: '╠', top: 2, right: 2, bottom: 2, ..Default::default()};
        let mut cell: Cell = Default::default();
        cell.set_possible(vec![trb]);
        cell.collapse();
        assert!(cell.content.is_some());
    }
}
