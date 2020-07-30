use clap::Clap;
use clap_num::si_number;

#[derive(Clap, Debug)]
struct Args {
    #[clap(short, long, parse(try_from_str=si_number))]
    resistance: u8,
}

fn main() {
    let args = Args::parse();
    println!("Resistance: {} ohms", args.resistance)
}
