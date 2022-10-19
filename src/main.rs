mod argparsing;
mod calculation;

use argparsing::Parser;

fn main() {
    let args = argparsing::Args::parse();
    println!(
        "Digit {} of PI is {}",
        args.digit,
        calculation::digit(args.digit as i32)
    );
}
