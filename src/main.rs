use std::{env};

mod arguments;
mod file_setup;

use arguments::Settings;

pub static DEFAULT_CXX_STANDARD: &str = "20";
pub static DEFAULT_MIN_CMAKE: &str = "3.20";

fn main() {
    if let Err(e) = run()
    {
        eprintln!("Runtime error: {e}");
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>>
{
    let args = env::args().collect::<Vec<_>>();
    let settings = Settings::from_args(args)?;

    println!("{:?}", settings);
    file_setup::setup_workspace(&settings)?;

    Ok(())
}