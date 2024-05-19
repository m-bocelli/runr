use std::convert::TryFrom;
use vmm::{ Vmm, VMMConfig };
use cli::{ parse_cli };
use image::{ extract };

fn main() {
    match parse_cli() {
        Ok(cmd_info) => {
            // if run cmd was used, launch VM
            if cmd_info.is_running {
                let mut rootfs = extract(cmd_info.image_name);
                rootfs = "path=".to_owned() + &rootfs;
                let mem = "size_mib=".to_owned() + &cmd_info.memory_size;

                launch(Some(&rootfs), Some(&mem));
            } 
        }
        Err(e) => {
            dbg!(e);
        }
    }
}

fn launch(rootfs_path: Option<&str>, memory: Option<&str>) {
    // try to build VMMConfig using defaults and the kernel we built from Makefile
    dbg!(rootfs_path);
    match VMMConfig::builder()
        .memory_config(memory)
        .vcpu_config(Some("num=1"))
        .kernel_config(Some("path=/var/lib/runr/kernel/vmlinux-5.10.210"))
        .net_config(Some("tap=tap0"))
        .block_config(rootfs_path) // root filesystem of the Docker image
        .build() {
        Ok(vmm_config) => {
            // launch VM
            let mut container = Vmm::try_from(vmm_config).expect("Failed to launch runr container");
            container.run().unwrap();
        }
        Err(e) => {
            dbg!(e);
        }
    }

}
