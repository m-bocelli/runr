use clap::{Command,Arg};
use image::extract; 
use std::result;

pub fn parse_cli() -> result::Result<&'static str, &'static str> {
    let matches = Command::new("runr")
        .arg_required_else_help(true)
        .author("ROOM2")
        .version("0.1.0")
        .about("Runs regular Docker images as Rust VMM containers using an Ubuntu 20.04 kernel.")
        .subcommand(Command::new("run") // using subcommands so we can expand CLI in the future
                    .about("Pulls and launches Docker images as runr containers")
                    .arg(
                        Arg::new("image")
                        .required(true)
                        .value_name("image")
                        .help("Docker image to build container from"))
                    .arg(
                        Arg::new("memory")
                        .long("memory")
                        .default_value("256")
                        .help("Amount of memory (in mib) dedicated to container")))
        .subcommand(Command::new("pull")
                    .about("Downloads image from Docker")
                    .arg(
                        Arg::new("image")
                        .required(true)
                        .value_name("image")
                        .help("Docker image to pull")))
        .get_matches();

    match matches.subcommand() {
        None => Err("No argument provided"),
        Some(("run", run_match)) => Ok("hi"), 
        Some(("pull", pull_match)) => Ok("hi"),
        Some(_) => Err("Invalid argument"),
    }
}
