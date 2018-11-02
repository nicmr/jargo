use std::env;
use std::fs;
use std::io;
use crate::project::{Project, ProjectBuilder};
use std::path::PathBuf;

/// Creates a new project directory, src subfolder and jango.toml
///
pub fn new_project(project_name: &str) -> std::io::Result<()> {
    use std::fs::{DirBuilder, File};
    use std::io::Write;

    //try to create the project directory, check if dir of name already exists
    let mut root = env::current_dir()?;
    root.push(project_name);
    println!("Trying to create '{}' project under '{}' ...", project_name, root.display());

    if let Err(e) = DirBuilder::new().create(root.clone()){
        println!("A directory with that name already exists."); 
        return Err(e);
    };


    //create Project struct
    let project = ProjectBuilder::new(project_name.to_string()).build();

    //create jargo.toml, which stores a serialized version of the project struct
    let mut tomlpath = root.clone();
    tomlpath.push("jargo");
    tomlpath.set_extension("toml");
    let mut toml = File::create(tomlpath)?;

    let serialized = project.serialize();
    match serialized{
        Ok(string) => toml.write_all(string.as_bytes())?,
        Err(e) => {
            println!("failed to serialise project settings. error message: {}", e)
        }
    }




    //create src dir
    let mut srcpath = root.clone();
    srcpath.push("src");

    //because the directory is guaranteed to not already contain a similarly named folder
    // (we just created it)
    // it is okay to cascade the ? here instead of matching the result it in place.
    DirBuilder::new().create(srcpath)?;

    let mut targetpath = root.clone();
    targetpath.push("target");
    //see src dir creation
    DirBuilder::new().create(targetpath)?;


    //create Main.java
    let mut mainpath = root.clone();
    mainpath.push("src");
    mainpath.push("Main");
    mainpath.set_extension("java");

    let mut f = File::create(mainpath)?;
    f.write_all(
br#"class Main{
    public static void main (String[] args){
        System.out.println("Hello world");
    }
}
"#)?;

    Ok(())
}

/// Checks if the current directory contains a valid project and runs  it
pub fn run_project(_pass_args: &str) -> std::io::Result<()> {
    //TODO: consider using different result type

    let current_dir = env::current_dir()?;


    //1. validate and compile
    if let Ok(_project) = compile_project(&current_dir, String::from("")){
        //2. run

        
    }
    
    Ok(())
}

/// Checks if the current directory contains a valid project and compiles it
pub fn compile_project(path: &PathBuf, _compiler_flags: String) -> std::io::Result<Project> {

    let project = check_project(path)?;
    project.compile()?;
    Ok(project)
}


/// Checks if the current directory contains a valid project
pub fn check_project(path: &PathBuf) -> std::io::Result<Project> {
    // iterate through current dirs
   
    let entries = fs::read_dir(path)?;
    for entry in entries {
        if let Ok(entry) = entry {
            //search for jargo.toml
            if entry.file_name() == "jargo.toml" {
                println!("valid project toml found at: {:?}", entry.path());
                return Ok(Project::parse_file(entry.path())?);
            }
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "unable to find project in current dir"))
}

/// Checks if the current directory contains a valid project, parses the toml and
// cleans the target directory
pub fn clean_project(path: PathBuf) -> std::io::Result<()> { 
    Ok(())
}