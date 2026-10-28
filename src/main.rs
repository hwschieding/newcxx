use std::{env, fs, io::{self, Write}};

mod arguments;

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
    src_dir_cmake_lists(&settings)?;

    Ok(())
}

fn write_base_cmake_lists(conf: &Settings) -> io::Result<()>
{
    let mut file = fs::File::create("CMakeLists.txt")?;

    writeln!(file, "cmake_minimum_required(VERSION {DEFAULT_MIN_CMAKE})")?;
    writeln!(file, "\nproject({} LANGUAGES CXX)", conf.project_name())?;

    if conf.compile_commands()
    {
        writeln!(file, "\nset(CMAKE_EXPORT_COMPILE_COMMANDS ON)")?;
    }

    writeln!(file, "\nset(CMAKE_CXX_STANDARD {})", conf.cxx_standard())?;
    writeln!(file, "set(CMAKE_CXX_STANDARD_REQUIRED ON)")?;
    writeln!(file, "set(CMAKE_CXX_EXTENSIONS OFF)")?;

    writeln!(file, "add_subdirectory(src)")?;

    Ok(())
}

fn src_dir_cmake_lists(conf: &Settings) -> io::Result<()>
{
    let mut file = fs::File::create("CMakeLists.txt")?;

    writeln!(file, "file(GLOB_RECURSE PROJECT_HEADERS CONFIGURE_DEPENDS \"include/*.h\")")?;
    
    writeln!(file, "\nadd_executable({})", conf.project_name())?;

    writeln!(file, "target_sources({}", conf.project_name())?;
    writeln!(file, "\tPRIVATE")?;
    writeln!(file, "\t\t\"main.cpp\"")?;
    writeln!(file, "\tPRIVATE FILE_SET HEADERS\n\t\tBASE_DIRS \"include\"")?;
    writeln!(file, "\t\tFILES\n\t\t\t${}\n)", "{PROJECT_HEADERS}")?;

    writeln!(file, "\n#target_link_libraries({} PRIVATE", conf.project_name())?;
    writeln!(file, "#\n#)")?;

    Ok(())
}

fn make_dirs() -> io::Result<()>
{
    fs::create_dir_all("src/include")
}