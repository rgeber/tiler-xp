use tilor_xp::cli::Args;
use clap::Parser;
use tilor_xp::{AppError, run};

fn main() -> Result<(), AppError>{
    let args = Args::parse();
    run(args)
}
