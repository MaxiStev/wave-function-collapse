mod cell;
mod cli;
mod field;
use clap::Parser;
use field::Field;

fn main() {
    println!("Hello, world!");

    let args = cli::Args::parse();

    let celops = cell::default::cells();
    let mut field = Field::new(args.height, args.width, celops);
    field.complete(args.progress);
    println!("{}", field);
}
