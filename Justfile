build_target := 'thumbv7em-none-eabihf'
features := 'fpu'

# Check for mistakes
lint:
	rustup target add {{build_target}}
	rustup component add clippy
	cargo clippy --package drone-cortex-m-macros
	cargo clippy --target {{build_target}} --features "{{features}}" --package drone-cortex-m

# Reformat the code
fmt:
	rustup component add rustfmt
	cargo fmt

# Generate the docs
doc:
	rustup target add {{build_target}}
	cargo doc --package drone-cortex-m-macros
	cargo doc --target {{build_target}} --features "{{features}}" --package drone-cortex-m

# Open the docs in a browser
doc_open: doc
	cargo doc --target {{build_target}} --features "{{features}}" --package drone-cortex-m --open

# Update README.md
readme:
	cargo readme -o README.md

# Run the tests
test:
	cargo test --package drone-cortex-m-macros
	cargo test --features "{{features}} std" --package drone-cortex-m
