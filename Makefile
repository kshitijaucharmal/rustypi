default:
	rm -rf target
	cargo b
	cp target/armv7a-none-eabi/debug/rustypi build/symbols
	arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/rustypi build/kernel7.img
