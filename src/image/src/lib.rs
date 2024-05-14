use std::env;

pub fn extract(image_name: String) {
    use std::process::Command;

    let pull_script_path = "./pull_docker_image.sh";
    Command::new(pull_script_path)
        .arg(&image_name)
        .output()
        .expect("failed to execute process");

    let run_script_path = "./create_rootfs.sh";
    let output = Command::new(run_script_path)
        .arg(&image_name) 
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_name = args[1].clone();
    extract(image_name);
}
