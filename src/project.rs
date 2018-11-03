use serde_derive::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;



#[derive(Debug, Clone)]
pub struct Project {
    data: ProjectData,
    absolute_path: PathBuf,
}
impl Project { 
    pub fn new(data: ProjectData, absolute_path: PathBuf) -> Project {
        Project{
            data,
            absolute_path
        }
    }

    /// Compiles files in self.data.src_dir and places binaries in self.data.target_dir
    /// Returns a result to indicate successful compilation
    pub fn compile(&self) -> std::io::Result<()> {
        use regex::Regex;
        use std::process::Command;

        // combine the absolute path of the jargo.toml with the relative src path
        let mut src_path = self.absolute_path.clone();
        src_path.push(&self.data.src_dir);
        

        // combine the absolute path of the jargo.toml with the relative target path
        let mut target_path = self.absolute_path.clone();
        target_path.push(&self.data.target_dir);


        if let Some(target_path_str) = target_path.to_str(){
            println!("target: {}", target_path_str);
            // match all .java files
            let src_entries = fs::read_dir(src_path)?;
            let re = Regex::new(r"[\w]+\.java").unwrap();

            for entry in src_entries {
                if let Ok(entry) = entry {
                    if let Some(filename) = &entry.file_name().to_str(){
                        if re.is_match(filename) {
                            println!("compiling {}...", filename);
                            Command::new("javac")
                                .arg(entry.path().to_str().unwrap())
                                .arg("-d")
                                .arg(target_path_str)
                                .output()
                                .expect("ERROR: javac failed to compile");
                        } else {
                            println!("ignoring {}", filename);
                        }
                    } else {
                        println!("unable to parse filename {:?}", entry.file_name());
                    }
                }
            }
            Ok(())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput,
            "Unable to convert target dir name to utf-8 str. dirname:"))
        }

            
    }

    /// Removes all files in self.data.target_dir (the target directory)
    pub fn clean(&self) -> std::io::Result<()>{

        println!("cleaning target directory...");
       
        let mut to_be_deleted: Vec<PathBuf> = Vec::new();

        let mut target_path = self.absolute_path.clone();
        target_path.push(&self.data.target_dir);
        let target_entries = fs::read_dir(target_path)?;

        for entry in target_entries {
            if let Ok(entry) = entry {
                to_be_deleted.push(entry.path())
            }
        }

        to_be_deleted
            .iter()
            .for_each(|x| println!("{:?}", x));

        // to_be_deleted
        //     .iter()
        //     .for_each(|x| {
        //         if let Err(e) = fs::remove_file(x){
        //             println!("Issue deleting file from target/ : {}", e)
        //         }     
        //     });
        Ok(())
    }
}




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectData {
    name: String,
    src_dir: String,
    target_dir: String,
    entry_point: String,
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
            entry_point:String::from("src/Main.java")
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
