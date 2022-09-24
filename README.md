[![crates.io](https://img.shields.io/crates/v/drone-cortexm.svg)](https://crates.io/crates/drone-cortexm)
![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# Drone Cortex-M

<!-- cargo-rdme start -->

ARM® Cortex®-M platform crate for Drone, an Embedded Operating System.

## Supported Cores

| Architecture | Core name              | Build target                | `drone_cortexm` Rust flag |
|--------------|------------------------|-----------------------------|---------------------------|
| ARMv6-M      | ARM® Cortex®-M0+ r0p0  | `thumbv6m-none-eabi`        | `cortexm0plus_r0p0`       |
| ARMv6-M      | ARM® Cortex®-M0+ r0p1  | `thumbv6m-none-eabi`        | `cortexm0plus_r0p1`       |
| ARMv7-M      | ARM® Cortex®-M3 r0p0   | `thumbv7m-none-eabi`        | `cortexm3_r0p0`           |
| ARMv7-M      | ARM® Cortex®-M3 r1p0   | `thumbv7m-none-eabi`        | `cortexm3_r1p0`           |
| ARMv7-M      | ARM® Cortex®-M3 r1p1   | `thumbv7m-none-eabi`        | `cortexm3_r1p1`           |
| ARMv7-M      | ARM® Cortex®-M3 r2p0   | `thumbv7m-none-eabi`        | `cortexm3_r2p0`           |
| ARMv7-M      | ARM® Cortex®-M3 r2p1   | `thumbv7m-none-eabi`        | `cortexm3_r2p1`           |
| ARMv7E-M     | ARM® Cortex®-M4 r0p0   | `thumbv7em-none-eabi`       | `cortexm4_r0p0`           |
| ARMv7E-M     | ARM® Cortex®-M4 r0p1   | `thumbv7em-none-eabi`       | `cortexm4_r0p1`           |
| ARMv7E-M     | ARM® Cortex®-M4F r0p0  | `thumbv7em-none-eabihf`     | `cortexm4f_r0p0`          |
| ARMv7E-M     | ARM® Cortex®-M4F r0p1  | `thumbv7em-none-eabihf`     | `cortexm4f_r0p1`          |
| ARMv8-M      | ARM® Cortex®-M33 r0p2  | `thumbv8m.main-none-eabi`   | `cortexm33_r0p2`          |
| ARMv8-M      | ARM® Cortex®-M33 r0p3  | `thumbv8m.main-none-eabi`   | `cortexm33_r0p3`          |
| ARMv8-M      | ARM® Cortex®-M33 r0p4  | `thumbv8m.main-none-eabi`   | `cortexm33_r0p4`          |
| ARMv8-M      | ARM® Cortex®-M33F r0p2 | `thumbv8m.main-none-eabihf` | `cortexm33f_r0p2`         |
| ARMv8-M      | ARM® Cortex®-M33F r0p3 | `thumbv8m.main-none-eabihf` | `cortexm33f_r0p3`         |
| ARMv8-M      | ARM® Cortex®-M33F r0p4 | `thumbv8m.main-none-eabihf` | `cortexm33f_r0p4`         |

Rust target triple and `drone_cortexm` Rust flag should be set at the
application level according to this table.

## Documentation

- [Drone Book](https://book.drone-os.com/)
- [API documentation](https://api.drone-os.com/drone-cortexm/0.15/)

## Usage

Add the crate to your `Cargo.toml` dependencies:

```toml
[dependencies]
drone-cortexm = { version = "0.15.0", features = [...] }
```

Add or extend `std` feature as follows:

```toml
[features]
std = ["drone-cortexm/std"]
```

<!-- cargo-rdme end -->

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
