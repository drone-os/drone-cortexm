# Changelog

This project follows semantic versioning.

Possible log types:

- `[added]` for new features.
- `[changed]` for changes in existing functionality.
- `[deprecated]` for once-stable features removed in upcoming releases.
- `[removed]` for deprecated features removed in this release.
- `[fixed]` for any bug fixes.
- `[security]` to invite users to upgrade in case of vulnerabilities.

### v0.12.0 (2020-05-01)

- [added] Added Cortex-M33 support
- [changed] Feature `fpu` renamed to `floating-point-unit`
- [added] Added feature `security-extension`
- [changed] Bit-band support moved behind the new `bit-band` feature
- [added] Added SECURE_FAULT exception mapping
- [changed] Added threads initialization token to `vtable!`
- [changed] Removed `thr::init!` macro in favor of
  `thr::init`/`thr::init_extended` functions
- [added] Added `memory-protection-unit` feature
- [removed] Removed `itm::trace_*` hooks
- [removed] Removed `dbg`, `eprint`, `eprintln`, `print`, `println` macros
- [removed] Removed `itm::*_PORT`, `itm::HEAP_TRACE_KEY` constants
- [removed] Removed `itm::write_str`, `itm::write_fmt` functions
- [removed] Removed `itm::update_prescaler!` macro in favor of
  `itm::update_prescaler` function
- [changed] Renamed `itm` module to `swo`
- [added] Added `swo::set_log!` macro
- [changed] Lang item definitions moved to `drone-core`
- [removed] Removed `prelude` module
- [changed] Renamed the whole crate to `drone-cortexm` (previously was
  `drone-cortex-m`)
- [changed] Renamed `cortex_m_core` config flag to `cortexm_core` as well as its
  values from `cortex_m*` to `cortexm*`

### v0.11.1 (2019-11-27)

- [changed] Upgraded to `syn` 1.0
- [changed] Using the newly released `futures` 0.3 instead of `futures-preview`
  0.3-alpha

### v0.11.0 (2019-11-06)

- [added] New macro `itm::update_prescaler!`
- [changed] `itm::update_prescaler` function now takes a single argument
- [changed] Switched to `cortex_m_core` config flag to specify the MCU core
  version

### v0.10.1 (2019-09-27)

- [fixed] Fixed API documentation by moving to self-hosted
  https://api.drone-os.com
