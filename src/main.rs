mod commands;

use clap::{Arg, App, SubCommand,};

fn main() {

    let matches = App::new("Jango - a minimalist java project manager")
                        .version("0.1")
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
                        .get_matches();
                

    if let Some(projname) = matches.value_of("new") {
        if let Err(_) = commands::new_project(projname){
            println!("Failed to create project.");
        };
    }

    if matches.is_present("run"){
        commands::run();
    }
}



