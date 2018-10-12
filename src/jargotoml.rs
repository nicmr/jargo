use serde_derive::{Serialize, Deserialize};
use toml;


///Reads the specified file to a String and parses it by calling `parse()`
pub fn parse_file(path: std::path::PathBuf) -> std::io::Result<()>{
    use std::fs::File;
    use std::io::Read;

    let mut contents = String::new();

    //TODO: add better error handling
    if let Ok(mut file) = File::open(path){
        file.read_to_string(&mut contents)?;
        parse(&contents);
    }
    Ok(())
}

/// Validates and parses the string and, if possible, returns a Project instance
//TODO: has to return results
pub fn parse(contents: &str) -> Project{

    //TODO: add better error handling
    let project: Project = toml::from_str(contents).unwrap();
    println!("{}", project.name);
    project
}


#[derive(Serialize, Deserialize)]
pub struct Project {
    name: String,
    src_dir: String,
    target_dir: String,
    entry_point: String,
}
impl Project{
    pub fn new(name: String, target_dir: String, entry_point: String) -> Project{
        Project{
            name,
            target_dir,
            entry_point,
        }
    }
}


//Builder pattern to achieve quasi-optional parameters

pub struct ProjectBuilder{
    name: String,
    src_dir: String,
    target_dir: String,
    entry_point: String,
}

impl ProjectBuilder{
    ///Returns a ProjectBuilder with default values, that can be modified and turn into a `Project`
    pub fn new(name: String) -> Project{
        Project{
            name,
            src_dir: String::from("src")
            target_dir: String::from("target")
            entry_point:String::from("src/Main.java")
        }
    }
    ///Consumes the current `ProjectBuilder` instance and returns a `Project` instance.
    pub fn build(self) -> Project {
        Project{
            name: self.name,
            src_dir: self.src_dir,
            target_dir: self.target_dir,
            entry_point: self.entry_point,
        }
    }


}
