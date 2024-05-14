KERN_PATH = /var/lib/runr/kernel
KERN_FILE = /var/lib/runr/kernel/vmlinux-5.10.210
UP_TO_DATE_MSG = "Kernel Already Downloaded: Not Installing"

install:
ifneq ("$(wildcard $(KERN_FILE))","")
        echo $(UP_TO_DATE_MSG)
else
        wget https://s3.amazonaws.com/spec.ccfc.min/firecracker-ci/v1.8/x86_64/vmlinux-5.10.210 -P $(KERN_PATH)
endif

clean:
        rm $(KERN_FILE)