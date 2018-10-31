use serde_derive::{Serialize, Deserialize};

///Reads the specified file to a String and parses it by calling `parse()`
pub fn parse_file(path: std::path::PathBuf) -> std::io::Result<Project>{
    use std::fs::File;
    use std::io::Read;

    let mut contents = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut contents)?;
    let project = parse(&contents);
    Ok(project)

}

/// Validates and parses the string and, if possible, returns a Project instance
//TODO: has to return results
pub fn parse(contents: &str) -> Project{

    //TODO: add better error handling
    let project: Project = toml::from_str(contents).unwrap();
    println!("Project name: {}", project.name);
    project
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    name: String,
    src_dir: String,
    target_dir: String,
    entry_point: String,
}
impl Project{
    // Returns a new `Project` instance
    pub fn new(name: String, src_dir: String, target_dir: String, entry_point: String) -> Project{
        Project{
            name,
            src_dir,
            target_dir,
            entry_point,
        }
    }

    /// Compiles self and returns a result with a reference to self
    pub fn compile(&self) -> std::io::Result<()> {
        Ok(())
    }

    pub fn serialize(&self) -> std::result::Result<String, toml::ser::Error> {
        let tomlstring = toml::to_string(&self)?;
        Ok(tomlstring)
    }
}


//Builder pattern to achieve quasi-optional parameters
#[derive(Debug, Clone)]
pub struct ProjectBuilder{
    name: String,
    src_dir: String,
    target_dir: String,
    entry_point: String,
}

impl ProjectBuilder{
    ///Returns a ProjectBuilder with default values, that can be modified and turn into a `Project`
    pub fn new(name: String) -> ProjectBuilder{
        ProjectBuilder{
            name,
            src_dir: String::from("src"),
            target_dir: String::from("target"),
            entry_point:String::from("src/Main.java")
        }
    }

    ///Consumes the current `ProjectBuilder` instance and returns a `Project` instance.
    pub fn build(self) -> Project {
        Project::new(self.name, self.src_dir, self.target_dir, self.entry_point)
    }

    //changes the src directory to the specified subpath, relative to `root`
    #[allow(dead_code)]
    pub fn set_src(&mut self, subpath: String) -> &mut ProjectBuilder{
        self.src_dir = subpath;
        self
    }
    #[allow(dead_code)]
    pub fn set_target(&mut self, subpath: String) -> &mut ProjectBuilder{
        self.target_dir = subpath;
        self
    }
    #[allow(dead_code)]
    pub fn set_entrypoint(&mut self, subpath: String) -> &mut ProjectBuilder{
        self.entry_point = subpath;
        self
    } 

}
