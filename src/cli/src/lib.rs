use clap::{Arg, Command};
use std::result;

/*
    CommandInfo is a struct that includes the name of the image provided when running runr.
    It also includes a boolean to know which command argument is being run. (run or pull)
*/
pub struct CommandInfo {
    pub image_name: String,
    pub memory_size: String,
    pub is_running: bool,
}

/* parse_cli():
    args: None
    return: Command Info Struct or &str

    make command for runr. give it arguments such as all of the subcommands it uses.
    creates subcommands run and pull and returns the struct based on the arguments provided.
*/
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

    /*
       Here we find all of the matches for our subcommand.
       If None, return no argument provided error.
       If the command argument is "run" return the struct with is_running true.
       If the command argument is "pull" return the struct with is_running false.
       If argument is provided but its not one of the known commands. return "Invalid argument."
    */
    match matches.subcommand() {
        None => Err("No argument provided"),
        Some(("run", run_match)) => {
            let CmdInfo = CommandInfo {
                image_name: run_match.clone().remove_one("image").expect("failed"),
                memory_size: run_match.clone().remove_one("memory").expect("failed"),
                is_running: true,
            };

            println!("{:?}", cmdInfo.image_name);
            return Ok(cmdInfo);
        }
        Some(("pull", pull_match)) => {
            let CmdInfo = CommandInfo {
                image_name: pull_match.clone().remove_one("image").expect("failed"),
                memory_size: pull_match.clone().remove_one("memory").expect("failed"),
                is_running: false,
            };

            return Ok(cmdInfo);
        }
        Some(_) => Err("Invalid argument"),
    }
}
