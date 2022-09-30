use clap::Parser;
use clap_num::si_number;

#[derive(Parser)]
struct Args {
    #[clap(short, long, value_parser=si_number::<u128>)]
    resistance: u128,
}

fn main() {
    let args = Args::parse();
    println!("Resistance: {} ohms", args.resistance)
}
