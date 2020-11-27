//! todo
#![deny(
    rust_2018_idioms,
    warnings,
    clippy::pedantic,
    missing_docs,
    missing_debug_implementations,
    broken_intra_doc_links,
    unsafe_code
)]
#![allow(incomplete_features)]
#![feature(const_generics)]

/// The `Unit` struct can represent every possible unit
/// that is defined in the [`SI`] system.
///
/// It is able to do so because it contains a list of all
/// 7 base units and a number which represents the exponent
/// of that unit.
///
/// # Example
///
/// ## Newton
/// ```no_rust
/// kg * m * s⁻²
/// ```
///
/// would be represented using the following `Unit`:
/// ```no_rust
/// Unit {
///     m: 1,
///     kg: 1,
///     s: -2,
/// }
/// ```
///
/// [`SI`]: https://jcgm.bipm.org/vim/en/1.16.html
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(non_snake_case)]
pub struct Unit {
    m: i8,
    kg: i8,
    s: i8,
    A: i8,
    K: i8,
    mol: i8,
    cd: i8,
}

/// A `Quantity` represents a raw value and it's unit
/// that is represented as a const generic parameter.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct Quantity<const U: Unit> {
    /// The raw value of this `Quantity`
    pub value: f64,
}
