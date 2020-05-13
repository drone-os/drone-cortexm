cortexm_core := 'cortexm4f_r0p1'
export DRONE_RUSTFLAGS := '--cfg cortexm_core="' + cortexm_core + '"'
target := 'thumbv7em-none-eabihf'
features := 'bit-band floating-point-unit memory-protection-unit security-extension'

# Install dependencies
deps:
	rustup target add {{target}}
	rustup component add clippy
	rustup component add rustfmt
	type cargo-readme >/dev/null || cargo +stable install cargo-readme

# Reformat the source code
fmt:
	cargo fmt

# Check the source code for mistakes
lint:
	cargo clippy --package drone-cortexm-macros
	drone env {{target}} -- cargo clippy --features "{{features}}" --package drone-cortexm

# Build the documentation
doc:
	cargo doc --package drone-cortexm-macros
	drone env {{target}} -- cargo doc --features "{{features}}" --package drone-cortexm

# Open the documentation in a browser
doc-open: doc
	drone env {{target}} -- cargo doc --features "{{features}}" --package drone-cortexm --open

# Run the tests
test:
	cargo test --package drone-cortexm-macros
	drone env -- cargo test --features "{{features}} std" --package drone-cortexm

# Update README.md
readme:
	cargo readme -o README.md

# Bump the versions
version-bump version drone-version drone-core-version:
	sed -i "s/\(api\.drone-os\.com\/drone-cortexm\/\)[0-9]\+\(\.[0-9]\+\)\+/\1$(echo {{version}} | sed 's/\(.*\)\.[0-9]\+/\1/')/" \
		Cargo.toml macros/Cargo.toml src/lib.rs
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[package\]/version = "{{version}}"/;t;x}' \
		Cargo.toml macros/Cargo.toml
	sed -i '/\[.*\]/h;/version = "=.*"/{x;s/\[.*drone-cortexm-.*\]/version = "={{version}}"/;t;x}' \
		Cargo.toml
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[.*drone-config\]/version = "{{drone-version}}"/;t;x}' \
		macros/Cargo.toml
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[.*drone\(-macros\)\?-core\]/version = "{{drone-core-version}}"/;t;x}' \
		Cargo.toml macros/Cargo.toml
	sed -i 's/\(drone-cortexm.*\)version = "[^"]\+"/\1version = "{{version}}"/' \
		src/lib.rs

# Publish to crates.io
publish:
	cd macros && cargo publish
	sleep 30
	drone env {{target}} -- cargo publish --features "{{features}}"

# Publish the docs to api.drone-os.com
publish-doc: doc
	dir=$(sed -n 's/.*api\.drone-os\.com\/\(.*\/.*\)\/.*\/"/\1/;T;p' Cargo.toml) \
		&& rm -rf ../drone-api/$dir \
		&& cp -rT target/doc ../drone-api/$dir \
		&& cp -rT target/{{target}}/doc ../drone-api/$dir \
		&& echo '<!DOCTYPE html><meta http-equiv="refresh" content="0; URL=./drone_cortexm">' > ../drone-api/$dir/index.html \
		&& cd ../drone-api && git add $dir && git commit -m "Docs for $dir"
