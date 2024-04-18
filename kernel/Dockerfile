FROM ubuntu:20.04

ENV KERN_ROOT /var/lib/runr/kernel

# This is just stolen from runq Im afraid to take any of it out but it installs things that we need and other stuff too probably
RUN echo "do_initrd = no" >> /etc/kernel-img.conf \
    && apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        cpio \
        e2fsprogs \
        git \
        libseccomp-dev \
        linux-virtual \
        pkg-config \
        qemu-system-x86 \
        rsync \
        wget \
        xz-utils

# Make the directory that a copy of the kernel will be stored in
RUN mkdir -p $KERN_ROOT

# Copy the kernel that the docker container is using to the kernel folder so that the make file can extract it
# And rename it to kernel
# So in the directory /var/lib/runr/kernel is a file called kernel that will be extracted as a tar file that the vmm reference will use
RUN cp /boot/vmlinuz-*-generic $KERN_ROOT/kernel
