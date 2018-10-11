use serde_derive::{Deserialize};
use toml;

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

pub fn parse(contents: &str) -> Project{

    //TODO: add better error handling
    let project: Project = toml::from_str(contents).unwrap();
    println!("{}", project.name);
    project
}

#[derive(Deserialize)]
pub struct Project {
    name: String,
}