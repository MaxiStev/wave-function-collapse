use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(short, value_parser, default_value_t = 20)]
    pub width: usize,
    #[clap(short, value_parser, default_value_t = 20)]
    pub height: usize,
    #[clap(long)]
    pub print: bool,
    #[clap(long)]
    pub progress: bool
}
