KERN_PATH = /var/lib/runr

install:	
	wget https://s3.amazonaws.com/spec.ccfc.min/firecracker-ci/v1.8/x86_64/vmlinux-5.10.210 -P $(KERN_PATH)

clean:
	rm $(KERN_PATH)/kernel
