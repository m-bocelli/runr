# runr

Rust hypervisor-based Docker runtime to launch regular Docker images as virtual machines instead of runc containers.

Goals:
- Reduce container escape probability
- Faster runtime than similar projects via leveraging RustVMM

# Installation

Prerequisites:
- KVM enabled host kernel (tested on Ubuntu 22.04)

```
# Grab the source code
git clone https://github.com/m-bocelli/runr

# Change into dir
cd runr

# Build the release binary
cargo build
```

# Running

```
# Launch a BusyBox Docker image inside a runr container
runr run busybox
```