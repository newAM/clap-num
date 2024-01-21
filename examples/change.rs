use clap::Parser;
use clap_num::number_range;
use std::borrow::Cow;

fn less_than_100(s: &str) -> Result<u8, Cow<'static, str>> {
    number_range(s, 0, 99)
}

#[derive(Parser)]
struct Change {
    #[clap(long, value_parser=less_than_100)]
    cents: u8,
}

fn main() {
    let args = Change::parse();
    println!("Change: {} cents", args.cents);
}
