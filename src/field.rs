use crate::cell::{Cell, CellContent, CellOp};
pub struct Field<'a> {
    pub field: array2d::Array2D<Cell<'a>>,
    pub allowed_cells: Vec<CellContent>
}
impl Field<'_> {
    pub fn new(height: usize, width: usize, allowed_cells: Vec<CellContent>) -> Self {
        let mut field = array2d::Array2D::filled_with(Cell::default(), height, width);
        Self {field, allowed_cells}
    }
    pub fn print(&self) {
        for row in self.field.as_rows().iter() {
            for col in row.iter() {
                print!("{}", if let Some(content) = col.content {content.content} else {'X'});
            }
            println!();
        }
    }
    fn surrounding(&self, row: usize, col: usize) -> (CellOp, CellOp, CellOp, CellOp) {
        let top: CellOp = None;
        let right: CellOp = None;
        let bottom: CellOp = None;
        let left: CellOp = None;
        if let Some(pos) = row.checked_sub(1) {
            if let Some(cell) = self.field.get(pos, col) {
                top = cell.content;
            }
        }
        if let Some(pos) = col.checked_add(1) {
            if let Some(cell) = self.field.get(row, pos) {
                right = cell.content;
            }
        }
        if let Some(pos) = row.checked_add(1) {
            if let Some(cell) = self.field.get(pos, col) {
                bottom = cell.content;
            }
        }
        if let Some(pos) = col.checked_sub(1) {
            if let Some(cell) = self.field.get(row, pos) {
                left = cell.content;
            }
        }
        (top, right, bottom, left)
    }
    fn lowest_entropy (&self) -> Vec<&Cell>{
        let field_ref = self.allowed_cells.iter()
            .map(|c| c)
            .collect::<Vec<&CellContent>>();
        let mut to_collapse = self.field.elements_row_major_iter()
            .filter(|c| c.content.is_some())
            .collect::<Vec<&Cell>>();

        // let mut lowest = usize::max_value();
        // for row in 0..self.field.row_len() {
        //     for col in 0..self.field.column_len() {
        //         if let Some(cell) = self.field.get(row, col) {
        //             if cell.content.is_none() {
        //                 lowest = lowest.min(Cell::possible_tuple(&field_ref, self.surrounding(row, col)).len());
        //             }
        //         }
        //     }
        // }
        let lowest = to_collapse.iter()
            .map(|_| Cell::possible_tuple(&field_ref, self.surrounding(1,1)).len())
            .reduce(|acc, item| {
                if acc < item {acc} else {item}
            }).expect("No item with entropy");
        to_collapse.retain(|cell| cell.poss2(&field_ref).len() == lowest);
        return to_collapse;
    }
}
