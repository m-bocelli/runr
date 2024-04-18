RUNR_ROOT = /var/lib/runr
KERN_ROOT := $(RUNR_ROOT)/kernel

image:
	docker build -t 2004kernel .

kernel.tar: image
	docker run --rm 2004kernel tar -C $(RUNR_ROOT) -cf - . > kernel.tar

install: kernel.tar
	mkdir -p /var/lib/runr/kernel
	tar -C /var/lib/runr/kernel -xf kernel.tar

clean:
	-rm -f kernel.tar
