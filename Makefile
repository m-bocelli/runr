install-kernel:
	$(MAKE) -C kernel install

release: install-kernel
	chmod +x src/image/src/pull_docker_image.sh
	chmod +x src/image/src/create_rootfs.sh
	cargo build --release

debug: 	install-kernel
	chmod +x src/image/src/pull_docker_image.sh
	chmod +x src/image/src/create_rootfs.sh
	cargo build

clean:
	${MAKE} -C kernel clean
	cargo clean
