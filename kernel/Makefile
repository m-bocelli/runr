KERN_IMG = 2004kernel
KERN_PATH = /var/lib/runr/kernel

# installs the kernel from the tar file
install: kernel.tar
	mkdir -p $(KERN_PATH)
	tar -C $(KERN_PATH) -xf kernel.tar

# extracts the kernel from the docker container as a tar file
kernel.tar: image
	docker run --rm $(KERN_IMG) tar -C $(KERN_PATH) -cf - . > kernel.tar

# makes the docker image that is used to install the kernel
image:
	docker build -t $(KERN_IMG) .

# removes things that dont take a long time to build
lazy-clean:
	rm -f kernel.tar
	rm -r $(KERN_PATH)

# true clean
clean: lazy-clean
	docker image rm $(KERN_IMG)
