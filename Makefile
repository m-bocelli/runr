install-kernel:
	$(MAKE) -C kernel install

release: install-kernel
	cargo build --release

debug: 	
	cargo build

clean:
	${MAKE} -C kernel clean
	cargo clean
