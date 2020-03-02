# Changelog

This project follows semantic versioning.

Possible log types:

- `[added]` for new features.
- `[changed]` for changes in existing functionality.
- `[deprecated]` for once-stable features removed in upcoming releases.
- `[removed]` for deprecated features removed in this release.
- `[fixed]` for any bug fixes.
- `[security]` to invite users to upgrade in case of vulnerabilities.

### Unreleased

- [added] Added Cortex-M33 support
- [changed] Feature `fpu` renamed to `floating-point-unit`
- [added] Added feature `security-extension`
- [changed] Bit-band support moved behind the new `bit-band` feature
- [added] Added SECURE_FAULT exception mapping

### v0.11.1 (2019-11-27)

- [changed] Upgraded to `syn` 1.0
- [changed] Using the newly released `futures` 0.3 instead of `futures-preview`
  0.3-alpha

### v0.11.0 (2019-11-06)

- [added] New macro `itm::update_prescaler!`
- [changed] `itm::update_prescaler` function now takes a single argument
- [changed] Using `cortex_m_core` config flag to specify the MCU core version

### v0.10.1 (2019-09-27)

- [fixed] Fix API documentation by moving to self-hosted https://api.drone-os.com
