use std::process::Command;
use std::env;

pub fn extract(image_name: String)-> String {
    let current_dir = env::current_dir().unwrap();
    let pull_path = current_dir.join("src/image/src/pull_docker_image.sh");
    let run_path = current_dir.join("src/image/src/create_rootfs.sh");

    Command::new(pull_path)
        .arg(&image_name)
        .output()
        .expect("failed to execute process");

    let output = Command::new(run_path)
        .arg(&image_name) 
        .output()
        .expect("failed to execute process");

    let mut rootfs_path = String::from_utf8(output.stdout).unwrap();
    rootfs_path.pop();
    return rootfs_path; 
}
