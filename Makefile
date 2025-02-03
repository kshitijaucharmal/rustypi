default:
	cargo build --release
	rust-objcopy -O binary target/aarch64-unknown-none/release/rpi3-baremetal kernel.img
