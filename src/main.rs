mod commands;
mod jargotoml;

use clap::{Arg, App,};
use std::path::PathBuf;

fn main() {

    let matches = App::new("Jargo - a minimalist java project manager")
                        .version("0.0.1")
                        .author("Nicolas Mohr <Nico.Mohr@gmx.net")
                        .about("Runs, tests and configures your javaproject from the command line")
                        //jargo --new PROJECT_NAME
                        .arg(Arg::with_name("new")
                            .short("n")
                            .long("new")
                            .case_insensitive(true)
                            .value_name("PROJECT_NAME")
                            .takes_value(true)
                            .help("creates a new jango project"))
                        //jargo --run
                        .arg(Arg::with_name("run")
                            .short("r")
                            .long("run")
                            .case_insensitive(true)
                            .takes_value(false)
                            .help("runs the current jango project"))
                        //jargo --build
                        .arg(Arg::with_name("build")
                            .short("b")
                            .long("build")
                            .case_insensitive(true)
                            .takes_value(false)
                            .help("builds the current jango project"))
                        //jargo --clean
                        .arg(Arg::with_name("clean")
                            .short("c")
                            .long("clean")
                            .case_insensitive(true)
                            .takes_value(false)
                            .help("cleans the current jango project target directory"))
                        .get_matches();
                

    if let Some(projname) = matches.value_of("new") {
        match commands::new_project(projname){
            Ok(_) => {
                println!("successfully created project");
            }
            Err(e) => {
                println!("Failed to create project. Error: {:?}", e);
            }
        };
    }

    //TODO: add {new, run, build} to mutually exclusive group

    if matches.is_present("run"){
        if let Err(_) = commands::run_project(""){
            println!("Failed to run project.")
        }
    }

    if matches.is_present("build"){
        if let Err(_) = commands::compile_project(PathBuf::from(""), String::from("")){
            println!("Failed to compile project.")
        }
    }

    if matches.is_present("clean"){
        if let Err(_) = commands::clean_project(PathBuf::from("")){
            println!("Failed to clean project.")
        }
    }
}



