#!/bin/bash
image_name=$1
targetdir="/var/lib/runr/${image_name}"
mkdir -p "${targetdir}"

outfile="${targetdir}/rootfs.ext4"

# Create ext4 filesystem on empty 1G file
dd if=/dev/zero of="${outfile}" bs=1M count=1024
mkfs.ext4 "${outfile}" >/dev/null 2>&1

# Create a temporary directory for mounting the filesystem
tmpdir="$(mktemp -d)"
trap "{ rm -rf $tmpdir; }" EXIT
mount "${outfile}" "${tmpdir}" >/dev/null 2>&1

# Run docker container and get sha
sha="$(docker run -d --entrypoint /bin/echo ${image_name > /dev/null 2>&1 })"

# Export docker container into mounted filesystem
docker export "${sha}" | tar -x -C "${tmpdir}" >/dev/null 2>&1

# Clean up
docker rm "${sha}" >/dev/null 2>&1
umount "${tmpdir}"

echo $targetdir
