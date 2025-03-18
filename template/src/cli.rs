use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Input file to process
    #[arg(short, long)]
    pub input: Option<String>,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}
