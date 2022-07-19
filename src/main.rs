mod cell;
mod field;
use field::Field;

fn main() {
    println!("Hello, world!");

    let celops = cell::default::cells();
    let mut field = Field::new(20, 20, celops);
    while field.to_collapse().len() > 0 {
        field.print();
        println!("----------");
        let cell = field.get_random_cell_to_collapse().unwrap().to_owned();
        field.collapse_cell(cell.row, cell.col);
    }
    field.print();
}
