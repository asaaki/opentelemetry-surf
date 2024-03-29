# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Additional changes to original format:
- `Cosmetic` for changes without impact on the code/logic
- `Thank you for your contribution` for shout-outs to the community

## [Unreleased]

## [0.6.0] - 2022-02-15
### Changed
- Update dependencies and fix breaking changes

  ```toml
  opentelemetry = "0.17"
  opentelemetry-semantic-conventions = "0.9"
  ```

  Due to lifetime and thread-safety issues (non-Send across await point),
  a switch to `BoxedTracer` was necessary. Since most examples and implementation do that,
  this crate gets in line with the others now.

  Examples are updated accordingly.

  Easiest way is to use `::default()` instead of `::new(tracer)` to use the global tracer.

  ```rust
  #[async_std::main]
  async fn main() -> surf::Result<()> {
      let _tracer = opentelemetry_jaeger::new_pipeline().install_batch(opentelemetry::runtime::AsyncStd)?;
      let otel_mw = opentelemetry_surf::OpenTelemetryTracingMiddleware::default();
      let client = surf::client().with(otel_mw);
      let res = client.get("https://httpbin.org/get").await?;
      dbg!(res);

      opentelemetry::global::shutdown_tracer_provider();
      Ok(())
  }
  ```

## [0.5.0] - 2021-10-03 — _German Unity Edition_
### Changed
- Update dependencies

  ```toml
  [dependencies]
  http-types = "2.12"
  opentelemetry = "0.16"
  opentelemetry-semantic-conventions = "0.8"
  surf = "2.3"
  ```

### Cosmetic
- Use Rust 1.54's new feature to include the README content into the crate doc via a macro call;
  see <https://blog.rust-lang.org/2021/07/29/Rust-1.54.0.html#attributes-can-invoke-function-like-macros>

## [0.4.0] - 2021-07-28
### Changed
- Update dependencies

## [0.3.0] - 2021-05-10
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

[Unreleased]: https://github.com/asaaki/opentelemetry-surf/compare/v0.6.0...HEAD
[0.6.0]: https://github.com/asaaki/opentelemetry-surf/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/asaaki/opentelemetry-surf/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/asaaki/opentelemetry-surf/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/asaaki/opentelemetry-surf/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/asaaki/opentelemetry-surf/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/asaaki/opentelemetry-surf/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/asaaki/opentelemetry-surf/commit/fceb3722ff2a317ce4b1e7d669978885d77105c5

[bbigras]: https://github.com/bbigras
