# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Additional changes to original format:
- `Cosmetic` for changes without impact on the code/logic
- `Thank you for your contribution` for shout-outs to the community

## [Unreleased]
### Changed
- Update dependencies and adapt code accordingly

### Cosmetic
- Ignore "RUSTSEC-2020-0056: stdweb is unmaintained" (#5)
- Ignore aes related audits until upstream dependencies have been updated
  - Ignore "RUSTSEC-2021-0059: `aesni` has been merged into the `aes` crate"
  - Ignore "RUSTSEC-2021-0060: `aes-soft` has been merged into the `aes` crate"
- Use cargo audit directly, as `actions-rs/audit-check` does not support ignore option

## [0.2.0] - 2021-04-03
### Changed
- Update dependencies and adapt code accordingly

  This is a breaking change!
  Most notably: The "uninstall" guard is gone; see examples for how to do it with current otel crates.

### Cosmetic
- github repo maintenance
- github actions improvements

## [0.1.1] - 2020-12-06
### Cosmetic
- documentation presentation improvements
- readme fixes

### Thank you for your contribution
- [bbigras]

## [0.1.0] - 2020-12-06
**Initial release**

[Unreleased]: https://github.com/asaaki/opentelemetry-surf/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/asaaki/opentelemetry-surf/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/asaaki/opentelemetry-surf/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/asaaki/opentelemetry-surf/commit/fceb3722ff2a317ce4b1e7d669978885d77105c5

[bbigras]: https://github.com/bbigras
