
use crate::DEFAULT_MIN_CMAKE;
use crate::arguments::Settings;
use std::{fs, io::{self, Write}, path::{Path, PathBuf}};

pub fn setup_workspace(conf: &Settings) -> io::Result<()>
{
    let proj_dir = create_project_dir(conf)?;
    make_dirs(&proj_dir)?;
    write_base_cmake_lists(conf, &proj_dir)?;
    src_dir_cmake_lists(conf, &proj_dir)?;
    make_project_file(&proj_dir)?;
    Ok(())
}

fn create_project_dir(conf: &Settings) -> io::Result<PathBuf>
{
    Ok(conf.out_dir().join(Path::new(conf.project_name())))
}

fn write_base_cmake_lists(conf: &Settings, proj_dir: &Path) -> io::Result<()>
{
    let mut file = fs::File::create(proj_dir.join("CMakeLists.txt"))?;

    writeln!(file, "cmake_minimum_required(VERSION {DEFAULT_MIN_CMAKE})")?;
    writeln!(file, "\nproject({} LANGUAGES CXX)", conf.project_name())?;

    if conf.compile_commands()
    {
        writeln!(file, "\nset(CMAKE_EXPORT_COMPILE_COMMANDS ON)")?;
    }

    writeln!(file, "\nset(CMAKE_CXX_STANDARD {})", conf.cxx_standard())?;
    writeln!(file, "set(CMAKE_CXX_STANDARD_REQUIRED ON)")?;
    writeln!(file, "set(CMAKE_CXX_EXTENSIONS OFF)")?;

    writeln!(file, "\nadd_subdirectory(src)")?;

    Ok(())
}

fn src_dir_cmake_lists(conf: &Settings, proj_dir: &Path) -> io::Result<()>
{
    let mut file = fs::File::create(proj_dir.join("src/CMakeLists.txt"))?;

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

fn make_dirs(proj_dir: &Path) -> io::Result<()>
{
    fs::create_dir_all(proj_dir.join(Path::new("src/include")))
}

fn make_project_file(proj_dir: &Path) -> io::Result<()>
{
    let mut file = fs::File::create(proj_dir.join("src/main.cpp"))?;

    writeln!(file, "#include <iostream>")?;
    writeln!(file, "\nint main(int argc, char* argv[])")?;
    writeln!(file, "{}", "{")?;
    writeln!(file, "    std::cout << \"Hello World!\\n\";")?;
    writeln!(file, "{}", "}")?;

    Ok(())
}