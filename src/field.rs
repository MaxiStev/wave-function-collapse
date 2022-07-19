use rand::Rng;

use crate::cell::{Cell, CellContent, CellOp};
pub struct Field {
    pub field: array2d::Array2D<Cell>,
    pub allowed_cells: Vec<CellContent>,
    rng: rand::prelude::ThreadRng,
}
impl Field {
    pub fn new(height: usize, width: usize, allowed_cells: Vec<CellContent>) -> Self {
        let mut field = array2d::Array2D::filled_with(Cell::default(), height, width);
        for row in 0..field.column_len() {
            for col in 0..field.row_len() {
                let mut cell = field.get_mut(row, col).unwrap();
                cell.row = row;
                cell.col = col;
                cell.set_possible(allowed_cells.to_owned());
            }
        }
        Self {
            field,
            allowed_cells,
            rng: rand::prelude::thread_rng(),
        }
    }
    pub fn print(&self) {
        for row in self.field.as_rows().iter() {
            for col in row.iter() {
                print!(
                    "{}",
                    if let Some(content) = col.content {
                        content.content
                    } else {
                        'X'
                    }
                );
            }
            println!();
        }
    }
    pub fn to_collapse(&self) -> Vec<&Cell> {
        self.field
            .elements_row_major_iter()
            .filter(|c| c.content.is_none())
            .collect::<Vec<&Cell>>()
    }
    pub fn lowest_entropy(&self) -> Vec<&Cell> {
        let mut to_collapse = self.to_collapse();

        let lowest = to_collapse
            .iter()
            .map(|c| c.entropy())
            .reduce(|acc, item| if acc < item { acc } else { item })
            .expect("No item with entropy");
        to_collapse.retain(|cell| cell.entropy() == lowest);
        return to_collapse;
    }

    pub fn get_random_cell_to_collapse(&self) -> Option<&Cell> {
        if self.to_collapse().len() > 0 {
            let to_collapse = self.lowest_entropy();
            let mut rng = rand::prelude::thread_rng();
            let cell = to_collapse
                .get(rng.gen_range(0..to_collapse.len()))
                .unwrap();
            Some(*cell)
        } else {
            None
        }
    }
    pub fn collapse_cell(&mut self, row: usize, col: usize) {
        let cont: CellContent;
        // let (top, right, bottom, left) = self.surrounding(row, col);
        let cell = self.field.get_mut(row, col).unwrap();
        cell.collapse(&mut self.rng);
        cont = cell.content.unwrap().to_owned();
        if let Some(pos) = row.checked_add(1) {
            if let Some(update_cell) = self.field.get_mut(pos, col) {
                update_cell.update_top(cont);
            }
        }
        if let Some(pos) = col.checked_sub(1) {
            if let Some(update_cell) = self.field.get_mut(row, pos) {
                update_cell.update_right(cont);
            }
        }
        if let Some(pos) = row.checked_sub(1) {
            if let Some(update_cell) = self.field.get_mut(pos, col) {
                update_cell.update_bottom(cont);
            }
        }
        if let Some(pos) = col.checked_add(1) {
            if let Some(update_cell) = self.field.get_mut(row, pos) {
                update_cell.update_left(cont);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cell::test::DefaultCellContent;
    #[test]
    fn lowest_entropy() {
        let def = DefaultCellContent::new();
        let mut field = Field::new(2, 2, def.vec());
        field.field.get_mut(0, 0).unwrap().content = Some(def.empty);
        field.field.get_mut(1, 0).unwrap().update_top(def.empty);
        field.field.get_mut(0, 1).unwrap().update_top(def.empty);
        assert_eq!(field.lowest_entropy().len(), 2);
    }

    #[test]
    fn collapse_update_neighbours() {
        let def = DefaultCellContent::new();
        let mut field = Field::new(2, 2, def.vec());
        field
            .field
            .get_mut(1, 1)
            .unwrap()
            .set_possible(vec![def.trb]);
        field.collapse_cell(1, 1);
        assert_eq!(field.field.get(0, 0).unwrap().possible.len(), 5);
        assert_eq!(field.field.get(1, 0).unwrap().possible.len(), 2);
        assert_eq!(field.field.get(0, 1).unwrap().possible.len(), 3);
    }
}
