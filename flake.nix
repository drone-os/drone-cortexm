{
  description = "ARM® Cortex®-M platform crate for Drone, an Embedded Operating System";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-22.05";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, utils, nixpkgs, fenix }:
    utils.lib.eachDefaultSystem (system:
      let
        buildTarget = "thumbv7em-none-eabihf";
        rustFlags = ''--cfg drone_cortexm="cortexm4f_r0p1"'';
        rustChannel = {
          channel = "nightly";
          date = "2022-11-12";
          sha256 = "NZrKSshDgITZuDSffP89NpZl/pQlblc7arXatkV+O9A=";
        };

        pkgs = nixpkgs.legacyPackages.${system};
        rustToolchain = with fenix.packages.${system}; combine
          ((with toolchainOf rustChannel; [
            rustc
            cargo
            clippy
            rustfmt
            rust-src
          ]) ++ (with targets.${buildTarget}.toolchainOf rustChannel; [
            rust-std
          ]));
        rustAnalyzer = fenix.packages.${system}.rust-analyzer;

        crossEnv = {
          CARGO_BUILD_TARGET = buildTarget;
        };
        nativeEnv = {
          CARGO_BUILD_TARGET = pkgs.stdenv.targetPlatform.config;
        };

        cargoRdme = (
          pkgs.rustPlatform.buildRustPackage rec {
            name = "cargo-rdme";
            src = pkgs.fetchFromGitHub {
              owner = "orium";
              repo = name;
              rev = "v0.7.3";
              sha256 = "qzit/uYkyWiOqpO5sHYo2hKJvOhovcO+oVbq/Bo2HsI=";
            };
            cargoSha256 = "lbyLVmSLNt4mt6hQbJnCuNL1Y1/2E/81sVpLYOkv7w8=";
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.openssl ];
            doCheck = false;
          });

        checkAll = pkgs.writeShellScriptBin "check-all" ''
          set -ex
          cargo rdme --check
          cargo fmt --all --check
          cargo clippy --workspace --exclude drone-cortexm-macros --features all -- --deny warnings
          nix develop '.#native' -c cargo clippy --package drone-cortexm-macros -- --deny warnings
          nix develop '.#native' -c cargo test --workspace --features all,host
          RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --workspace --exclude drone-cortexm-macros --features all
          RUSTDOCFLAGS='-D warnings' nix develop '.#native' -c cargo doc --no-deps --package drone-cortexm-macros
        '';

        updateVersions = pkgs.writeShellScriptBin "update-versions" ''
          sed -i "s/\(api\.drone-os\.com\/drone-cortexm\/\)[0-9]\+\(\.[0-9]\+\)\+/\1$(echo $1 | sed 's/\(.*\)\.[0-9]\+/\1/')/" \
            Cargo.toml macros/Cargo.toml src/lib.rs
          sed -i "/\[.*\]/h;/version = \".*\"/{x;s/\[workspace.package\]/version = \"$1\"/;t;x}" \
            Cargo.toml
          sed -i "/\[.*\]/h;/version = \"=.*\"/{x;s/\[.*drone-cortexm-.*\]/version = \"=$1\"/;t;x}" \
            Cargo.toml
          sed -i "/\[.*\]/h;/version = \".*\"/{x;s/\[.*drone\(-macros\)\?-core\]/version = \"$2\"/;t;x}" \
            Cargo.toml
          sed -i "/\[.*\]/h;/version = \".*\"/{x;s/\[.*drone-config\]/version = \"$3\"/;t;x}" \
            Cargo.toml
          sed -i "s/\(drone-cortexm.*\)version = \"[^\"]\+\"/\1version = \"$1\"/" \
            src/lib.rs
        '';

        publishCrates = pkgs.writeShellScriptBin "publish-crates" ''
          cd macros && nix develop '.#native' -c cargo publish
          sleep 30
          cargo publish --features all
        '';

        publishDocs = pkgs.writeShellScriptBin "publish-docs" ''
          dir=$(sed -n 's/.*api\.drone-os\.com\/\(.*\/.*\)\/.*\/"/\1/;T;p' Cargo.toml) \
            && rm -rf ../drone-api/$dir \
            && cp -rT target/doc ../drone-api/$dir \
            && cp -rT target/$CARGO_BUILD_TARGET/doc ../drone-api/$dir \
            && echo '<!DOCTYPE html><meta http-equiv="refresh" content="0; URL=./drone_cortexm">' > ../drone-api/$dir/index.html \
            && cd ../drone-api && git add $dir && git commit -m "Docs for $dir"
        '';

        mkShell = extraEnv: pkgs.mkShell ({
          nativeBuildInputs = [
            rustToolchain
            rustAnalyzer
            cargoRdme
            checkAll
            updateVersions
            publishCrates
            publishDocs
          ];
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          CARGO_BUILD_RUSTFLAGS = rustFlags;
        } // extraEnv);
      in
      {
        devShells = rec {
          cross = mkShell (crossEnv // { name = "cross"; });
          native = mkShell (nativeEnv // { name = "native"; });
          default = cross;
        };
      }
    );
}
