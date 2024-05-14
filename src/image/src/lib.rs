use std::process::Command;

pub fn extract(image_name: String)-> String {

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

    return String::from_utf8_lossy(&output.stdout).to_string();
}

