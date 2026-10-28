use crate::DEFAULT_CXX_STANDARD;

struct SettingsBuilder
{
    project_name_build: Option<String>,
    cxx_standard_build: String,
    compile_commands_build: bool,
}

impl SettingsBuilder
{
    fn new() -> Self
    {
        Self {
            project_name_build: None,
            cxx_standard_build: String::from(DEFAULT_CXX_STANDARD),
            compile_commands_build: false,
        }
    }
    fn set_name(&mut self, new: Option<String>) -> Result<(), &'static str>
    {
        match new
        {
            Some(s) => { self.project_name_build = Some(s); Ok(()) }
            None => { Err("No name specified for arg") }
        }
    }
    fn set_standard(&mut self, new: Option<String>) -> Result<(), &'static str>
    {
        match new
        {
            Some(s) => { self.cxx_standard_build = s; Ok(()) }
            None => { Err("No standard specified for arg") }
        }
    }
    fn set_compile_commands(&mut self, new: bool) -> ()
    {
        self.compile_commands_build = new;
    }
    fn build(self) -> Result<Settings, &'static str>
    {
        match self.project_name_build
        {
            Some(name) => Ok(Settings{
                project_name: name,
                cxx_standard: self.cxx_standard_build,
                compile_commands: self.compile_commands_build,
            }),
            None => Err("No name specified")
        }
    }
}

#[derive(Debug)]
pub struct Settings
{
    project_name: String,
    cxx_standard: String,
    compile_commands: bool,
}

impl Settings
{
    pub fn from_args(raw_args: Vec<String>) -> Result<Self, &'static str>
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
}