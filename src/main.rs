//use std::env;
use cli::parse_cli;
use vmm::Vmm;

fn main() {
    match parse_cli() {
        Ok(image) => println!("working image: {image:?}"),
        Err(e) => eprintln!("Failed to parse cmdl. {}", e),
    };
}
