use clap::{Arg, Command};
use image::extract;
use std::result;

pub struct CommandInfo {
    pub image_name: String,
    is_running: bool,
}

pub fn parse_cli() -> result::Result<CommandInfo, &'static str> {
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

    //println!("Value for --output: {}", matches.get_one::<String>("runr").unwrap());
    match matches.subcommand() {
        None => Err("No argument provided"),
        Some(("run", run_match)) => {
            let cmdInfo = CommandInfo {
                image_name: run_match.clone().remove_one("image").expect("failed"),
                is_running: true,
            };

            println!("{:?}", cmdInfo.image_name);
            return Ok(cmdInfo);
        }
        Some(("pull", pull_match)) => {
            let cmdInfo = CommandInfo {
                image_name: pull_match.clone().remove_one("image").expect("failed"),
                is_running: false,
            };

            return Ok(cmdInfo);
        }
        Some(_) => Err("Invalid argument"),
    }
}
