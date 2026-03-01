
debug:
	cargo build

release:
	cargo build --release

check:
	cargo check 

demo: release
	timeout 4  ./target/release/cjp2p-rust                                         562b168a64967fd64687664b987dd1c50c36d1532449bb4c385d683538c0bf03 || true
	./target/release/cjp2p-rust $$(cat                                                shared/562b168a64967fd64687664b987dd1c50c36d1532449bb4c385d683538c0bf03 )
	cat $$(cat                                                shared/562b168a64967fd64687664b987dd1c50c36d1532449bb4c385d683538c0bf03 ) |sha256sum
	echo should be  6f5a06b0a8b83d66583a319bfa104393f5e52d2c017437a1b425e9275576500c


all: check release debug
