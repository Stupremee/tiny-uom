//! `tiny-uom` is a small version of the [`uom`] library.
//!
//! This crate is mostly based on [this] proof-of-concept using const generics.
//! `tiny-uom` is a port of `uom` to use const generics and to be a faster and smaller version.
//! It provides type-safe and zero-cost [dimensional-analysis].
//! `tiny-uom` provides all units that are specified in the [International System of Units][SI]
//! and all quantities that are specified in the [International System of Quantities][ISQ].
//!
//! ## Usage
//! ```
//! #![feature(const_generics, const_evaluatable_checked)]
//! #![allow(incomplete_features)]
//! use tiny_uom::values::{kg, m, s};
//!
//! # fn main() {
//! let distance = 10.0 * m;
//! let time = 2.0 * s;
//!
//! let velocity = distance / time;
//! assert_eq!(velocity, 5.0 * (m / s));
//! # }
//! ```
//!
//! [`uom`]: https://docs.rs/uom
//! [this]: https://docs.rs/const_unit_poc
//! [dimensional-analysis]: https://en.wikipedia.org/wiki/Dimensional_analysis
//! [SI]: https://jcgm.bipm.org/vim/en/1.16.html
//! [ISQ]: https://jcgm.bipm.org/vim/en/1.6.html
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
#![feature(const_generics, const_evaluatable_checked)]

use std::{fmt, ops};

mod si;
pub use si::{units, values};

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
    pub(crate) m: i8,
    pub(crate) kg: i8,
    pub(crate) s: i8,
    pub(crate) A: i8,
    pub(crate) K: i8,
    pub(crate) mol: i8,
    pub(crate) cd: i8,
}

impl Unit {
    /// Invert this unit by negating all exponents.
    pub const fn inv(self) -> Self {
        Self {
            m: -self.m,
            kg: -self.kg,
            s: -self.s,
            A: -self.A,
            K: -self.K,
            mol: -self.mol,
            cd: -self.cd,
        }
    }

    /// Multiply two units and return the resulting unit.
    pub const fn mul(self, rhs: Self) -> Self {
        Self {
            m: self.m + rhs.m,
            kg: self.kg + rhs.kg,
            s: self.s + rhs.s,
            A: self.A + rhs.A,
            K: self.K + rhs.K,
            mol: self.mol + rhs.mol,
            cd: self.cd + rhs.cd,
        }
    }

    /// Divide two units and return the resulting unit.
    pub const fn div(self, rhs: Self) -> Self {
        Self {
            m: self.m - rhs.m,
            kg: self.kg - rhs.kg,
            s: self.s - rhs.s,
            A: self.A - rhs.A,
            K: self.K - rhs.K,
            mol: self.mol - rhs.mol,
            cd: self.cd - rhs.cd,
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let units = [
            ("m", self.m),
            ("kg", self.kg),
            ("s", self.s),
            ("A", self.A),
            ("K", self.K),
            ("mol", self.mol),
            ("cd", self.cd),
        ];

        let units = units.iter().filter(|unit| unit.1 != 0);
        let len = units.clone().count();

        for (idx, (name, unit)) in units.enumerate() {
            if *unit == 1 {
                write!(f, "{}", name)?;
            } else {
                write!(f, "{}^{}", name, unit)?;
            }
            if idx + 1 != len {
                write!(f, " * ")?;
            }
        }

        Ok(())
    }
}

impl ops::Mul<Unit> for Unit {
    type Output = Self;

    /// Multiplies two units by adding their exponents.
    fn mul(self, rhs: Unit) -> Self::Output {
        self.mul(rhs)
    }
}

impl ops::Div<Unit> for Unit {
    type Output = Self;

    /// Divides two units by substracting their exponents.
    fn div(self, rhs: Unit) -> Self::Output {
        self.div(rhs)
    }
}

/// A `Quantity` represents a raw value and it's unit
/// that is represented as a const generic parameter.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct Quantity<const U: Unit> {
    /// The raw value of this `Quantity`
    pub value: f64,
}

/// Implement all methods and traits for a quantity type.
macro_rules! quantity_impl {
    ($num:ty, $t:ident) => {
        impl<const U: Unit> $t<U> {
            /// Create a new `Quantity` with the given value.
            pub const fn new(value: $num) -> Self {
                Self { value }
            }
        }

        impl<const U: Unit> ::std::fmt::Display for $t<U> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{} * {}", self.value, U)
            }
        }

        // ============================
        // Add implementations
        // ============================
        impl<const U: Unit> ::std::ops::Add<$t<U>> for $t<U> {
            type Output = Self;

            /// Add the value of two equal units.
            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    value: self.value + rhs.value,
                }
            }
        }

        impl<const U: Unit> ::std::ops::AddAssign<$t<U>> for $t<U> {
            /// Add the value of two equal units.
            fn add_assign(&mut self, rhs: Self) {
                self.value += rhs.value;
            }
        }

        // ============================
        // Sub implementations
        // ============================
        impl<const U: Unit> ::std::ops::Sub<$t<U>> for $t<U> {
            type Output = Self;

            /// Subtract the value of two equal units.
            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    value: self.value - rhs.value,
                }
            }
        }

        impl<const U: Unit> ::std::ops::SubAssign<$t<U>> for $t<U> {
            /// Subtract the value of two equal units.
            fn sub_assign(&mut self, rhs: Self) {
                self.value -= rhs.value;
            }
        }

        // ============================
        // Mul implementations
        // ============================
        impl<const U: Unit> ::std::ops::Mul<$num> for $t<U> {
            type Output = Self;

            /// Multiply the value of this unit with a number.
            fn mul(self, rhs: $num) -> Self::Output {
                Self {
                    value: self.value * rhs,
                }
            }
        }

        impl<const U: Unit> ::std::ops::Mul<$t<U>> for $num {
            type Output = $t<U>;

            /// Multiply the value of this unit with a number.
            fn mul(self, rhs: $t<U>) -> Self::Output {
                $t {
                    value: self * rhs.value,
                }
            }
        }

        impl<const L: Unit, const R: Unit> ::std::ops::Mul<$t<R>> for $t<L>
        where
            $t<{ L.mul(R) }>: ,
        {
            type Output = $t<{ L.mul(R) }>;

            /// Multiply two units and their values
            fn mul(self, rhs: $t<R>) -> Self::Output {
                $t {
                    value: self.value * rhs.value,
                }
            }
        }

        impl<const U: Unit> ::std::ops::MulAssign<$num> for $t<U> {
            /// Multiply the value of this unit with a number.
            fn mul_assign(&mut self, rhs: $num) {
                self.value *= rhs;
            }
        }

        // ============================
        // Div implementations
        // ============================
        impl<const U: Unit> ::std::ops::Div<$num> for $t<U> {
            type Output = Self;

            /// Divides the value of this unit with a number.
            fn div(self, rhs: $num) -> Self::Output {
                Self {
                    value: self.value / rhs,
                }
            }
        }

        impl<const L: Unit, const R: Unit> ::std::ops::Div<$t<R>> for $t<L>
        where
            $t<{ L.div(R) }>: ,
        {
            type Output = $t<{ L.div(R) }>;

            /// Divides two units and their values.
            fn div(self, rhs: $t<R>) -> Self::Output {
                $t {
                    value: self.value / rhs.value,
                }
            }
        }

        impl<const U: Unit> ::std::ops::Div<$t<U>> for $num
        where
            $t<{ U.inv() }>: ,
        {
            type Output = $t<{ U.inv() }>;

            fn div(self, rhs: $t<U>) -> Self::Output {
                $t {
                    value: self / rhs.value,
                }
            }
        }

        impl<const U: Unit> ::std::ops::DivAssign<$num> for $t<U> {
            /// Divides the value of this unit with a number.
            fn div_assign(&mut self, rhs: $num) {
                self.value /= rhs;
            }
        }
    };
}

quantity_impl!(f64, Quantity);
