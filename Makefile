default:
	rm -rf target
	cargo b
	cp target/armv7a-none-eabi/debug/rustypi symbols
	arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/rustypi kernel7.img
