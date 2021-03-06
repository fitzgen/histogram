# histogram - histogram storage and percentile stats

histogram is a stats library for rust which provides histogram
storage with percentile stats. Maintains precision guarentees
throughout the range of stored values.

[![travis-badge][]][travis] [![downloads-badge][] ![release-badge][]][crate] [![license-badge][]](#license)

[travis-badge]: https://img.shields.io/travis/brayniac/histogram/master.svg
[downloads-badge]: https://img.shields.io/crates/d/histogram.svg
[release-badge]: https://img.shields.io/crates/v/histogram.svg
[license-badge]: https://img.shields.io/crates/l/histogram.svg
[travis]: https://travis-ci.org/brayniac/histogram
[crate]: https://crates.io/crates/histogram
[Cargo]: https://github.com/rust-lang/cargo

## Usage

To use `histogram`, first add this to your `Cargo.toml`:

```toml
[dependencies]
histogram = "*"
```

Then, add this to your crate root:

```rust
extern crate histogram;
```

The API documentation of this library can be found at
[docs.rs/histogram](https://docs.rs/histogram/).

## Features

* Values are stored with precision guarentees
* Pre-allocated on initialization
* Retrieve percentile stats

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
