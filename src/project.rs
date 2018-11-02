use serde_derive::{Serialize, Deserialize};
use std::fs;



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    name: String,
    src_dir: String,
    target_dir: String,
    entry_point: String,
}
impl Project{
    /// Returns a new `Project` instance
    pub fn new(name: String, src_dir: String, target_dir: String, entry_point: String) -> Project{
        Project{
            name,
            src_dir,
            target_dir,
            entry_point,
        }
    }


    ///Reads the specified file to a String and parses it by calling `parse()`
    pub fn parse_file(path: std::path::PathBuf) -> std::io::Result<Project>{
        use std::fs::File;
        use std::io::Read;

        let mut contents = String::new();
        let mut file = File::open(path)?;

        file.read_to_string(&mut contents)?;
        let project = Project::parse_string(&contents)?;
        Ok(project)

    }

    /// Validates and parses the string and, if possible, returns a Project instance
    //TODO: has to return results
    pub fn parse_string(contents: &str) -> std::io::Result<Project>{

        //TODO: add better error handling
        let project: Project = toml::from_str(contents).unwrap();

        println!("Project name: {}", project.name);
        Ok(project)
    }

    /// Compiles files in self.src_dir and places binaries in self.target_dir
    /// Returns a result to indicate successful compilation
    pub fn compile(&self) -> std::io::Result<()> {
        use regex::Regex;

        let entries = fs::read_dir(&self.src_dir)?;

        let re = Regex::new(r"[\w]+\.java").unwrap();
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(filename) = &entry.file_name().to_str(){
                    //match .java files
                    if re.is_match(filename) {
                        println!("compiling {}...", filename);
                    } else {
                        println!("ignoring {}", filename);
                    }
                } else {
                    println!("unable to parse filename {:?}", entry.file_name());
                }
            }
        }
        Ok(())

    }

    pub fn clean(&self) -> std::io::Result<()>{
        Ok(())
    }

    pub fn serialize(&self) -> std::result::Result<String, toml::ser::Error> {
        let tomlstring = toml::to_string(&self)?;
        Ok(tomlstring)
    }
}


/// Builder pattern struct to achieve quasi-optional parameters for struct `Project`
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
