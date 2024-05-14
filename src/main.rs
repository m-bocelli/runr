use std::convert::TryFrom;
use vmm::{ Vmm, VMMConfig };


fn main() {
    // call parseCLI
    // call extract

    // try to build VMMConfig using defaults and the kernel we built from Makefile
    match VMMConfig::builder()
        .memory_config(Some("size_mib=2048"))
        .vcpu_config(Some("num=1"))
        .kernel_config(Some("path=/var/lib/runr/vmlinux-5.10.210"))
        .net_config(Some("tap=tap0"))
        .block_config(Some("path=/var/lib/runr/memcached/rootfs.ext4")) // root filesystem of the Docker image
        .build() {
            Ok(vmm_config) => {
                // launch VM
                let mut container = Vmm::try_from(vmm_config).expect("Failed to launch runr container");
                container.run().unwrap();
            }
            Err(e) => {
                dbg!("bad");
            }
        }
}
