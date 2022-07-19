mod cell;
mod field;
mod cli;
use field::Field;
use clap::Parser;

fn main() {
    println!("Hello, world!");

    let args = cli::Args::parse();

    let celops = cell::default::cells();
    let mut field = Field::new(args.height, args.width, celops);
    field.complete(args.print, args.progress);
    field.print();
}
