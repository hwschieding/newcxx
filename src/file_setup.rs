
use crate::DEFAULT_MIN_CMAKE;
use crate::arguments::Settings;

use git2::{Repository};
use tokio::{task::JoinSet, fs, io::AsyncWriteExt};

use std::{io::{self, Write}, path::{Path, PathBuf}, sync::Arc};

pub async fn setup_workspace(conf: Arc<Settings>) -> Result<(), Box<dyn std::error::Error>>
{
    let proj_dir = Arc::new(get_project_dir(&Arc::clone(&conf))?);
    let mut set = JoinSet::<()>::new();

    make_dirs(&Arc::clone(&proj_dir)).await?;

    let dir_clone = Arc::clone(&proj_dir);
    let conf_clone = Arc::clone(&conf);
    spawn_task(&mut set, async move{
        write_base_cmake_lists(&conf_clone, &dir_clone).await
    });

    let dir_clone = Arc::clone(&proj_dir);
    let conf_clone = Arc::clone(&conf);
    spawn_task(&mut set, async move {
        src_dir_cmake_lists(&conf_clone, &dir_clone).await
    });

    let dir_clone = Arc::clone(&proj_dir);
    spawn_task(&mut set, async move {
        make_project_file(&dir_clone).await
    });

    let dir_clone = Arc::clone(&proj_dir);
    spawn_task(&mut set, async move {
        init_git_repo(&dir_clone).await
    });

    set.join_all().await;

    Ok(())
}

fn spawn_task<F, T>(set: &mut JoinSet<()>, task: F) -> ()
where
    F: Future<Output = Result<(), T>>,
    F: Send + 'static,
    T: std::error::Error,
{
    set.spawn(async move {
        if let Err(e) = task.await
        {
            eprintln!("{e}");
        };

        ()
    });
}

fn get_project_dir(conf: &Settings) -> io::Result<PathBuf>
{
    Ok(conf.out_dir().join(Path::new(conf.project_name())))
}

async fn write_base_cmake_lists(conf: &Settings, proj_dir: &Path) -> io::Result<()>
{
    let mut file = fs::File::create(proj_dir.join("CMakeLists.txt")).await?;
    let mut buff = Vec::<u8>::new();

    writeln!(&mut buff, "cmake_minimum_required(VERSION {DEFAULT_MIN_CMAKE})")?;
    writeln!(&mut buff, "\nproject({} LANGUAGES CXX)", conf.project_name())?;

    if conf.compile_commands()
    {
        writeln!(&mut buff, "\nset(CMAKE_EXPORT_COMPILE_COMMANDS ON)")?;
    }

    writeln!(&mut buff, "\nset(CMAKE_CXX_STANDARD {})", conf.cxx_standard())?;
    writeln!(&mut buff, "set(CMAKE_CXX_STANDARD_REQUIRED ON)")?;
    writeln!(&mut buff, "set(CMAKE_CXX_EXTENSIONS OFF)")?;

    writeln!(&mut buff, "\nadd_subdirectory(src)")?;

    file.write_all(&buff).await
}

async fn src_dir_cmake_lists(conf: &Settings, proj_dir: &Path) -> io::Result<()>
{
    let mut file = fs::File::create(proj_dir.join("src/CMakeLists.txt")).await?;
    let mut buff = Vec::<u8>::new();


    writeln!(&mut buff, "file(GLOB_RECURSE PROJECT_HEADERS CONFIGURE_DEPENDS \"include/*.h\")")?;
    
    writeln!(&mut buff, "\nadd_executable({})", conf.project_name())?;

    writeln!(&mut buff, "target_sources({}", conf.project_name())?;
    writeln!(&mut buff, "\tPRIVATE")?;
    writeln!(&mut buff, "\t\t\"main.cpp\"")?;
    writeln!(&mut buff, "\tPRIVATE FILE_SET HEADERS\n\t\tBASE_DIRS \"include\"")?;
    writeln!(&mut buff, "\t\tFILES\n\t\t\t${}\n)", "{PROJECT_HEADERS}")?;

    writeln!(&mut buff, "\n#target_link_libraries({} PRIVATE", conf.project_name())?;
    writeln!(&mut buff, "#\n#)")?;

    file.write_all(&buff).await
}

async fn init_git_repo(proj_dir: &Path) -> Result<(), git2::Error>
{
    Repository::init(proj_dir)?;
    Ok(())
}

async fn make_dirs(proj_dir: &Path) -> io::Result<()>
{
    fs::create_dir_all(proj_dir.join(Path::new("src/include"))).await
}

async fn make_project_file(proj_dir: &Path) -> io::Result<()>
{
    let mut file = fs::File::create(proj_dir.join("src/main.cpp")).await?;
    let mut buff = Vec::<u8>::new();


    writeln!(&mut buff, "#include <iostream>")?;
    writeln!(&mut buff, "\nint main(int argc, char* argv[])")?;
    writeln!(&mut buff, "{}", "{")?;
    writeln!(&mut buff, "    std::cout << \"Hello World!\\n\";")?;
    writeln!(&mut buff, "{}", "}")?;

    file.write_all(&buff).await
}