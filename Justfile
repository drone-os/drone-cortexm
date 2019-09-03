build_target := 'thumbv7em-none-eabihf'
features := 'cortex_m4_r0p1 fpu'

# Install dependencies
deps:
	rustup component add clippy
	rustup component add rustfmt
	rustup component add rls rust-analysis rust-src
	type cargo-readme >/dev/null || cargo +stable install cargo-readme

# Reformat the source code
fmt:
	cargo fmt

# Check for mistakes
lint:
	cargo clippy --package drone-cortex-m-macros
	cargo clippy --target {{build_target}} --features "{{features}}" --package drone-cortex-m

# Generate the docs
doc:
	cargo doc --package drone-cortex-m-macros
	cargo doc --target {{build_target}} --features "{{features}}" --package drone-cortex-m

# Open the docs in a browser
doc-open: doc
	cargo doc --target {{build_target}} --features "{{features}}" --package drone-cortex-m --open

# Run the tests
test:
	cargo test --package drone-cortex-m-macros
	cargo test --features "{{features}} std" --package drone-cortex-m

# Update README.md
readme:
	cargo readme -o README.md

# Publish to crates.io
publish:
	cd macros && cargo publish
	cargo publish --target {{build_target}} --features "{{features}}"
