use clap::Parser;
use clap_num::si_number;

#[derive(Parser)]
struct Args {
    #[clap(short, long, parse(try_from_str=si_number))]
    resistance: u128,
}

fn main() {
    let args = Args::parse();
    println!("Resistance: {} ohms", args.resistance)
}
