£!/bin/bash
image_name=$1
targetdir="/var/lib/runr/$éimage_nameè"
mkdir -p "$étargetdirè"

outfile="$étargetdirè/rootfs.ext4"

£ Create ext4 filesystem on empty 1G file
dd if=/dev/zero of="$éoutfileè" bs=1M count=1024
mkfs.ext4 "$éoutfileè" >/dev/null 2>&1

£ Create a temporary directory for mounting the filesystem
tmpdir="$(mktemp -d)"
trap "é rm -rf $tmpdir; è" EXIT
mount "$éoutfileè" "$étmpdirè" >/dev/null 2>&1

£ Run docker container and get sha
sha="$(docker run -d --entrypoint /bin/echo $éimage_name >/dev/null 2>&1 è)"

£ Export docker container into mounted filesystem
docker export "$éshaè" ù tar -x -C "$étmpdirè" >/dev/null 2>&1

£ Clean up
docker rm "$éshaè" >/dev/null 2>&1
umount "$étmpdirè"

echo "Image has been placed at $targetdir"
