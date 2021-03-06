use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectData {
    pub name: String,
    pub src_dir: String,
    pub target_dir: String,
    pub entry_point: String,
}
impl ProjectData{
    /// Returns a new `ProjectData` instance
    pub fn new(name: String, src_dir: String, target_dir: String, entry_point: String) -> ProjectData{
        ProjectData{
            name,
            src_dir,
            target_dir,
            entry_point,
        }
    }


    ///Reads the specified file to a String and parses it by calling `parse()`
    pub fn parse_file(path: std::path::PathBuf) -> std::io::Result<ProjectData>{
        use std::fs::File;
        use std::io::Read;

        let mut contents = String::new();
        let mut file = File::open(path)?;

        file.read_to_string(&mut contents)?;
        let project_data = ProjectData::parse_string(&contents)?;
        Ok(project_data)

    }

    /// Validates and parses the string and, if possible, returns a ProjectData instance
    //TODO: has to return results
    pub fn parse_string(contents: &str) -> std::io::Result<ProjectData>{

        //TODO: add better error handling
        let project_data: ProjectData = toml::from_str(contents).unwrap();

        println!("ProjectData name: {}", project_data.name);
        Ok(project_data)
    }

    pub fn serialize(&self) -> std::result::Result<String, toml::ser::Error> {
        let tomlstring = toml::to_string(&self)?;
        Ok(tomlstring)
    }
}


/// Builder pattern struct to achieve quasi-optional parameters for struct `ProjectData`
#[derive(Debug, Clone)]
pub struct ProjectDataBuilder{
    name: String,
    src_dir: String,
    target_dir: String,
    entry_point: String,
}

impl ProjectDataBuilder{
    ///Returns a ProjectDataBuilder with default values, that can be modified and turn into a `ProjectData`
    pub fn new(name: String) -> ProjectDataBuilder{
        ProjectDataBuilder{
            name,
            src_dir: String::from("src"),
            target_dir: String::from("target"),
            entry_point:String::from("Main.java")
        }
    }

    ///Consumes the current `ProjectDataBuilder` instance and returns a `ProjectData` instance.
    pub fn build(self) -> ProjectData {
        ProjectData::new(self.name, self.src_dir, self.target_dir, self.entry_point)
    }

    //changes the src directory to the specified subpath, relative to `root`
    #[allow(dead_code)]
    pub fn set_src(&mut self, subpath: String) -> &mut ProjectDataBuilder{
        self.src_dir = subpath;
        self
    }
    #[allow(dead_code)]
    pub fn set_target(&mut self, subpath: String) -> &mut ProjectDataBuilder{
        self.target_dir = subpath;
        self
    }
    #[allow(dead_code)]
    pub fn set_entrypoint(&mut self, subpath: String) -> &mut ProjectDataBuilder{
        self.entry_point = subpath;
        self
    } 

}