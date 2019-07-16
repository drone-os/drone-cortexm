build_target := 'thumbv7em-none-eabihf'
test_target := 'thumbv7em-linux-eabihf'

# Check with clippy.
clippy:
	cargo clippy --package drone-cortex-m-macros
	cargo clippy --target {{build_target}} --all --exclude drone-cortex-m-macros

# Generate documentation.
doc:
	cargo doc --package drone-cortex-m-macros
	cargo doc --target {{build_target}} --all --exclude drone-cortex-m-macros

# Generate README.md from src/lib.rs.
readme:
	cargo readme -o README.md

# Run tests.
test:
	cargo test --package drone-cortex-m-macros
	RUST_TARGET_PATH=$(pwd) CROSS_COMPILE=arm-none-eabi- \
		xargo test --target {{test_target}} --package drone-cortex-m
