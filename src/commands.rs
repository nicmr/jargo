use std::env;
use crate::jargotoml;
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

    
    //create jargo.toml
    let mut tomlpath = root.clone();
    tomlpath.push("jargo");
    tomlpath.set_extension("toml");

    //create src dir
    let mut srcpath = root.clone();
    srcpath.push("src");

    //because the directory is guaranteed to not already contain a similarly named folder
    //(we just created it), and we can't handle the obscure (fs) errors that may still cause it to fail,
    // it is okay to cascade the ? here instead of matching the result it in place.
    DirBuilder::new().create(srcpath)?;

    let mut targetpath = root.clone();
    targetpath.push("target");
    //see src dir creation
    DirBuilder::new().create(targetpath);


    //create Main.java
    let mut mainpath = root.clone();
    mainpath.push("src");
    mainpath.push("Main");
    mainpath.set_extension("java");

    let mut f = File::create(root)?;
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
pub fn run_project(pass_args: &str) -> std::io::Result<()> {
    //TODO: consider using different result type

    use std::fs;

    let current_dir = env::current_dir()?;


    //1. validate current directory
    if let Ok(_) = check_project(current_dir){
        //2. compile project
        compile_project(current_dir)
    }
    

    //3. run java
    

    Ok(())
}

///Checks if the current directory contains a valid project and compiles it
pub fn compile_project(path: PathBuf, compiler_flags: String) -> std::io::Result<()> {

}


///Checks if the current directory contains a valid project
pub fn check_project(path: PathBuf) -> std::io::Result<()> {


    //iterate through current dir
    if let Ok(entries) = fs::read_dir(current_dir){
        for entry in entries { 
            if let Ok(entry) = entry {
                //search for jargo.toml
                if entry.file_name() == "jargo.toml" {
                    println!("valid project root found");
                    jargotoml::parse_file(entry.path());
                }
            }
        }
    }
}

/// Checks if the current directory contains a valid project, parses the toml and
// cleans the target directory
pub fn clean_project(path: PathBuf) -> std::io::Result<()> { 
    
}