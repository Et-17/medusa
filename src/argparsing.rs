pub use clap;
pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(help = "The digit of PI to compute")]
    pub digit: u32,
}
