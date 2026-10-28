use std::{env};

mod arguments;
mod file_setup;

use arguments::RunMode;

pub static DEFAULT_CXX_STANDARD: &str = "20";
pub static DEFAULT_MIN_CMAKE: &str = "3.20";

fn main() {
    if let Err(e) = run()
    {
        eprintln!("Runtime error: {e}");
        help();
        std::process::exit(1)
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>>
{
    let args = env::args().collect::<Vec<_>>();
    let settings = match RunMode::from_args(args)?
    {
        RunMode::Normal(s) => s,
        RunMode::Help => { help(); return Ok(()); }
    };

    println!("{:?}", settings);
    file_setup::setup_workspace(&settings)?;

    Ok(())
}

fn help() -> ()
{
    println!("{} v{}
\nOPTIONS
  [-n|--name] <project name>: Name of the new project (required)
  [-s|--standard] <c++ standard>: C++ standard (default: 20)

  --compcommands: enable 'CMAKE_EXPORT_COMPILE_COMMANDS'

  -o <directory>: output directory (default: working dir)

  [-h|-help|--help]: display this prompt",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
}