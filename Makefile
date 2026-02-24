
debug:
	cargo build

release:
	cargo build --release

check:
	cargo check 

demo: 
	timeout 4  ./target/release/cjp2p-rust                                         80dfa275db78819616ae2cf361c39323d3b2d69b565dfea5706eed6a29aad352 || true
	./target/release/cjp2p-rust $$(cat                                                shared/80dfa275db78819616ae2cf361c39323d3b2d69b565dfea5706eed6a29aad352 )
	cat $$(cat                                                shared/80dfa275db78819616ae2cf361c39323d3b2d69b565dfea5706eed6a29aad352 ) |sha256sum
	echo is c7dce40a2af023d2ab7d4bc26fac78cba7f7cb7854f67f9fb5bf72b14d9931d8


all: check release debug
