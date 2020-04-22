cortex_m_core := 'cortex_m4f_r0p1'
export DRONE_RUSTFLAGS := '--cfg cortex_m_core="' + cortex_m_core + '"'
target := 'thumbv7em-none-eabihf'
features := 'bit-band floating-point-unit memory-protection-unit security-extension'
cargo_features := '-Z features=itarget,build_dep,dev_dep -Z package-features'

# Install dependencies
deps:
	rustup target add {{target}}
	rustup component add clippy
	rustup component add rustfmt
	type cargo-readme >/dev/null || cargo +stable install cargo-readme

# Reformat the source code
fmt:
	cargo {{cargo_features}} fmt

# Check for mistakes
lint:
	cargo {{cargo_features}} clippy --package drone-cortex-m-macros
	drone env {{target}} -- cargo {{cargo_features}} clippy --features "{{features}}" --package drone-cortex-m

# Generate the docs
doc:
	cargo {{cargo_features}} doc --package drone-cortex-m-macros
	drone env {{target}} -- cargo {{cargo_features}} doc --features "{{features}}" --package drone-cortex-m

# Open the docs in a browser
doc-open: doc
	drone env {{target}} -- cargo {{cargo_features}} doc --features "{{features}}" --package drone-cortex-m --open

# Run the tests
test:
	cargo {{cargo_features}} test --package drone-cortex-m-macros
	drone env -- cargo {{cargo_features}} test --features "{{features}} std" --package drone-cortex-m

# Update README.md
readme:
	cargo {{cargo_features}} readme -o README.md

# Bump crate versions
version-bump version drone-version drone-core-version:
	sed -i "s/\(api\.drone-os\.com\/drone-cortex-m\/\)[0-9]\+\(\.[0-9]\+\)\+/\1$(echo {{version}} | sed 's/\(.*\)\.[0-9]\+/\1/')/" \
		Cargo.toml macros/Cargo.toml src/lib.rs
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[package\]/version = "{{version}}"/;t;x}' \
		Cargo.toml macros/Cargo.toml
	sed -i '/\[.*\]/h;/version = "=.*"/{x;s/\[.*drone-cortex-m-.*\]/version = "={{version}}"/;t;x}' \
		Cargo.toml
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[.*drone-config\]/version = "{{drone-version}}"/;t;x}' \
		macros/Cargo.toml
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[.*drone\(-macros\)\?-core\]/version = "{{drone-core-version}}"/;t;x}' \
		Cargo.toml macros/Cargo.toml
	sed -i 's/\(drone-cortex-m.*\)version = "[^"]\+"/\1version = "{{version}}"/' \
		src/lib.rs

# Publish to crates.io
publish:
	cd macros && cargo {{cargo_features}} publish
	sleep 5
	drone env {{target}} -- cargo {{cargo_features}} publish --features "{{features}}"

# Publish the docs to api.drone-os.com
publish-doc: doc
	dir=$(sed -n 's/.*api\.drone-os\.com\/\(.*\/.*\)\/.*\/"/\1/;T;p' Cargo.toml) \
		&& rm -rf ../drone-api/$dir \
		&& cp -rT target/doc ../drone-api/$dir \
		&& cp -rT target/{{target}}/doc ../drone-api/$dir \
		&& echo '<!DOCTYPE html><meta http-equiv="refresh" content="0; URL=./drone_cortex_m">' > ../drone-api/$dir/index.html \
		&& cd ../drone-api && git add $dir && git commit -m "Docs for $dir"
