use clap::{Arg, Command};
use image::extract;
use std::result;

pub fn parse_cli() -> result::Result<String, &'static str> {
    let matches = Command::new("runr")
        .arg_required_else_help(true)
        .author("ROOM2")
        .version("0.1.0")
        .about("Runs regular Docker images in Rust VMM instances using Ubuntu 20.04 kernel.")
        .subcommand(
            Command::new("run") // using subcommands so we can expand CLI in the future
                .about("Pulls and launches Docker images as runr containers")
                .arg(
                    Arg::new("image")
                        .required(true)
                        .value_name("image")
                        .help("Docker image to build container from"),
                )
                .arg(
                    Arg::new("memory")
                        .long("memory")
                        .default_value("256")
                        .help("Amount of memory (in mib) dedicated to container"),
                ),
        )
        .subcommand(
            Command::new("pull")
                .about("Downloads image from Docker")
                .arg(
                    Arg::new("image")
                        .required(true)
                        .value_name("image")
                        .help("Docker image to pull"),
                ),
        )
        .get_matches();

    // match matches.subcommand() {
    //     None => Err("No argument provided"),
    //     Some(("run", run_match)) => println!("{:?}", run_match),
    // }

    //let my_struct_string = format!("{:?}", matches.subcommand);

    // matches.subcommand() {
    //     Some(("run", run_match)) => {println!("Swag")};
    // }

    //println!("{}", my_struct_string);
    /*
            if let Some(c) = matches.get_one::<String>("runr") {
        println!("Value for -c: {c}");
    }
     */

    //println!("Value for --output: {}", matches.get_one::<String>("runr").unwrap());
    match matches.subcommand() {
        None => Err("No argument provided"),
        Some(("run", run_match)) => {
            let argstr: String = run_match.clone().remove_one("image").expect("failed");
            println!("{:?}", argstr);
            return Ok(argstr);
        }
        Some(("pull", pull_match)) => {
            let newstr: String = "Hello".to_string();
            return Ok(newstr);
        }
        Some(_) => Err("Invalid argument"),
    }
}
