FROM ubuntu:20.04

ENV RUNR_ROOT /var/lib/runr
ENV KERN_ROOT /var/lib/runr/kernel

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

RUN mkdir -p $KERN_ROOT

RUN cp /boot/vmlinuz-*-generic $RUNR_ROOT