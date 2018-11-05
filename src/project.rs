pub mod projectdata;

use std::fs;
use std::path::PathBuf;
use self::projectdata::{ProjectData};

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

    pub fn run(&self) -> std::io::Result<()> {
        use std::process::Command;
        use std::str;

        // Convert entry point from FILENAME.java to FILENAME
        let len = self.data.entry_point.len();
        let mut entry_point = self.data.entry_point.clone();
        entry_point.replace_range(len.saturating_sub(4).., "");
        println!("entry point: {}", entry_point);

        // Assemble the absolute path of target path to add it to the class path
        let mut target_path = self.absolute_path.clone();
        target_path.push(&self.data.target_dir);
        println!("target path: {}", target_path.to_str().unwrap());

        let java_output = Command::new("java")
            .arg("-cp")
            .arg(target_path.to_str().unwrap()) //TODO: FIX evil unwrap
            .arg(entry_point)
            .output();

        match java_output { 
            Ok(output) => {
                println!("stdout: {:?}", str::from_utf8(&output.stdout));
                println!("stderr: {:?}", str::from_utf8(&output.stderr));
            },
            Err(e) => println!("java process crashed with error: {}", e),
        };
        Ok(())
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

        // to_be_deleted
        //     .iter()
        //     .for_each(|x| println!("{:?}", x));

        to_be_deleted
            .iter()
            .for_each(|x| {
                if let Err(e) = fs::remove_file(x){
                    println!("Issue deleting file from target/ : {}", e)
                }     
            });
        Ok(())
    }
}