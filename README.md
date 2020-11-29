`tiny-uom`
==========
[![Crates.io](https://img.shields.io/crates/v/tiny-uom.svg)](https://crates.io/crates/tiny-uom)
[![Documentation](https://img.shields.io/badge/documentation-docs.rs-blue.svg)](https://docs.rs/tiny-uom)

**smol and fast version of [`uom`]**

[Documentation][docs-rs] | [Crate][crates-io] | [Examples][examples]

This crate is mostly based on [this] proof-of-concept using const generics.
`tiny-uom` is a port of `uom` to use const generics and to be a faster and smaller version.
It provides type-safe and zero-cost [dimensional-analysis].
`tiny-uom` provides all units that are specified in the [International System of Units][SI]
and all quantities that are specified in the [International System of Quantities][ISQ].

## Usage

`tiny-uom` requires nightly Rust compiler (currently it's always tested with `nightly-2020-11-25`).
Add this your `Cargo.toml`:

```toml
[dependencies]
tiny-uom = "0.1.0"
```

```rust
#![feature(const_generics, const_evaluatable_checked)]
#![allow(incomplete_features)]

use tiny_uom::values::{kg, m, s};

let distance = 10.0 * m;
let time = 2.0 * s;

let velocity = distance / time;
assert_eq!(velocity, 5.0 * (m / s));
```

### License

This project is licensed under the [MIT][license] license

[`uom`]: https://docs.rs/uom
[docs-rs]: https://docs.rs/tiny-uom
[crates-io]: https://crates.io/crates/tiny-uom
[examples]: https://github.com/Stupremee/tiny-uom/tree/main/examples
[license]: https://github.com/Stupremee/tiny-uom/tree/main/LICENSE
[this]: https://docs.rs/const_unit_poc
[dimensional-analysis]: https://en.wikipedia.org/wiki/Dimensional_analysis
[SI]: https://jcgm.bipm.org/vim/en/1.16.html
[ISQ]: https://jcgm.bipm.org/vim/en/1.6.html
