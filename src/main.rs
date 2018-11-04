mod commands;
mod project;

use clap::{Arg, App,};
use std::env;

fn main() {

    let matches = App::new("Jargo - a minimalist java project manager")
                        .version("0.0.1")
                        .author("Nicolas Mohr <Nico.Mohr@gmx.net>")
                        .about("Run, test and configure your java project from the command line")
                        //jargo --new PROJECT_NAME
                        .arg(Arg::with_name("new")
                            .short("n")
                            .long("new")
                            .case_insensitive(true)
                            .value_name("PROJECT_NAME")
                            .takes_value(true)
                            .help("Creates a new jargo project"))
                        //jargo --run
                        .arg(Arg::with_name("run")
                            .short("r")
                            .long("run")
                            .case_insensitive(true)
                            .takes_value(false)
                            .help("Runs the current jargo project"))
                        //jargo --build
                        .arg(Arg::with_name("build")
                            .short("b")
                            .long("build")
                            .case_insensitive(true)
                            .takes_value(false)
                            .help("Builds the current jargo project"))
                        //jargo --check
                        .arg(Arg::with_name("check")
                            .long("check")
                            .case_insensitive(true)
                            .takes_value(false)
                            .help("Recursively searches for a valid jargo.toml file in the current dir"))
                        //jargo --clean
                        .arg(Arg::with_name("clean")
                            .short("c")
                            .long("clean")
                            .case_insensitive(true)
                            .takes_value(false)
                            .help("Cleans the current jargo project target directory"))
                        .get_matches();
                

    let currentdir = env::current_dir();

    match currentdir{
        Ok(currentdir) => {
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
                    println!("Failed to run project.");
                }
            }

            if matches.is_present("build"){
                if let Err(_) = commands::compile_project(&currentdir, String::from("")){
                    println!("Failed to compile project.");
                }
            }

            if matches.is_present("clean"){
                if let Err(_) = commands::clean_project(&currentdir){
                    println!("Failed to clean project.");
                }
            }

            if matches.is_present("check"){
                if let Err(_) = commands::check_project(&currentdir){
                    println!("Failed to find a jargo.toml.");
                }
            }
        }
        Err(e) => {
            println!("Can't read in the current directory.\n
                        Please make sure all permissions are set correctly.", );
            println!("Error message: {}", e); 
        }
    }
    
    
}



