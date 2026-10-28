use std::{env, fmt, io, path::{Path, PathBuf}};

use crate::{DEFAULT_CXX_STANDARD};

#[derive(Debug)]
pub enum SettingsError
{
    MissingArg(&'static str),
    BadFile(io::Error),
}

impl fmt::Display for SettingsError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self
        {
            Self::MissingArg(msg) => write!(f, "{msg}"),
            Self::BadFile(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for SettingsError
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self
        {
            Self::BadFile(err) => Some(err),
            _ => None
        }
    }
}

impl From<io::Error> for SettingsError
{
    fn from(value: io::Error) -> Self {
        Self::BadFile(value)
    }
}

struct SettingsBuilder
{
    project_name_build: Option<String>,
    cxx_standard_build: Option<String>,
    compile_commands_build: bool,
    output_directory: Option<PathBuf>,
}

fn default_out_dir() -> io::Result<PathBuf>
{
    env::current_dir()
}

impl SettingsBuilder
{
    fn new() -> Self
    {
        Self {
            project_name_build: None,
            cxx_standard_build: None,
            compile_commands_build: false,
            output_directory: None,
        }
    }
    fn set_name(&mut self, new: Option<String>) -> Result<(), SettingsError>
    {
        match new
        {
            Some(s) => { self.project_name_build = Some(s); Ok(()) }
            None => { Err(
                SettingsError::MissingArg("No name specified for arg")
            ) }
        }
    }
    fn set_standard(&mut self, new: Option<String>) -> Result<(), SettingsError>
    {
        match new
        {
            Some(s) => { self.cxx_standard_build = Some(s); Ok(()) }
            None => { Err(
                SettingsError::MissingArg("No standard specified for arg")
            ) }
        }
    }
    fn set_compile_commands(&mut self, new: bool) -> ()
    {
        self.compile_commands_build = new;
    }
    fn set_out_dir(&mut self, new: Option<String>) -> Result<(), SettingsError>
    {
        match new
        {
            Some(s) => {
                self.output_directory = Some(default_out_dir()?.join(Path::new(&s)));
                Ok(())
            }
            None => Err(SettingsError::MissingArg("No directory specified for arg"))
        }
    }
    fn build(self) -> Result<Settings, SettingsError>
    {
        match self.project_name_build
        {
            Some(name) => Ok(Settings{
                project_name: name,
                cxx_standard: self.cxx_standard_build.unwrap_or(
                    String::from(DEFAULT_CXX_STANDARD)
                ),
                compile_commands: self.compile_commands_build,
                out_dir: self.output_directory.unwrap_or(default_out_dir()?)
            }),
            None => Err(SettingsError::MissingArg("No name specified"))
        }
    }
}

#[derive(Debug)]
pub struct Settings
{
    project_name: String,
    cxx_standard: String,
    compile_commands: bool,
    out_dir: PathBuf,
}

impl Settings
{
    pub fn from_args(raw_args: Vec<String>) -> Result<Self, SettingsError>
    {
        let mut builder = SettingsBuilder::new();

        let mut args_iter = raw_args.into_iter();
        args_iter.next();
        while let Some(arg) = args_iter.next()
        {
            match arg.to_lowercase().as_str()
            {
                "--name" => builder.set_name(args_iter.next())?,
                "-n" => builder.set_name(args_iter.next())?,
                "--standard" => builder.set_standard(args_iter.next())?,
                "-s" => builder.set_standard(args_iter.next())?,
                "--compcommand" => builder.set_compile_commands(true),
                "-o" => builder.set_out_dir(args_iter.next())?,
                _ => { println!("Unrecognized argument") },
            }
        }
        
        builder.build()
    }

    pub fn project_name(&self) -> &String
    {
        &self.project_name
    }

    pub fn cxx_standard(&self) -> &String
    {
        &self.cxx_standard
    }

    pub fn compile_commands(&self) -> bool
    {
        self.compile_commands
    }

    pub fn out_dir(&self) -> &PathBuf
    {
        &self.out_dir
    }
}