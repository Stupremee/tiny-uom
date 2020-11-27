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
#![feature(const_generics, const_evaluatable_checked)]

use std::ops;

pub mod units;
pub mod values;

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
            m: self.m + rhs.m,
            kg: self.kg + rhs.kg,
            s: self.s + rhs.s,
            A: self.A + rhs.A,
            K: self.K + rhs.K,
            mol: self.mol + rhs.mol,
            cd: self.cd + rhs.cd,
        }
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

// TODO: Display impl

/// Implement all methods and traits for a quantity type.
macro_rules! quantity_impl {
    ($num:ty, $t:ident) => {
        impl<const U: Unit> Quantity<U> {
            /// Create a new `Quantity` with the given value.
            pub fn new(value: f64) -> Self {
                Self { value }
            }
        }

        // ============================
        // Add implementations
        // ============================
        use std::ops::{Add, AddAssign};

        impl<const U: Unit> Add<$t<U>> for $t<U> {
            type Output = Self;

            /// Add the value of two equal units.
            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    value: self.value + rhs.value,
                }
            }
        }

        impl<const U: Unit> AddAssign<$t<U>> for $t<U> {
            /// Add the value of two equal units.
            fn add_assign(&mut self, rhs: Self) {
                self.value += rhs.value;
            }
        }

        // ============================
        // Sub implementations
        // ============================
        use std::ops::{Sub, SubAssign};

        impl<const U: Unit> Sub<$t<U>> for $t<U> {
            type Output = Self;

            /// Subtract the value of two equal units.
            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    value: self.value - rhs.value,
                }
            }
        }

        impl<const U: Unit> SubAssign<$t<U>> for $t<U> {
            /// Subtract the value of two equal units.
            fn sub_assign(&mut self, rhs: Self) {
                self.value -= rhs.value;
            }
        }

        // ============================
        // Mul implementations
        // ============================

        use std::ops::{Mul, MulAssign};

        impl<const U: Unit> Mul<$num> for $t<U> {
            type Output = Self;

            /// Multiply the value of this unit with a number.
            fn mul(self, rhs: $num) -> Self::Output {
                Self {
                    value: self.value * rhs,
                }
            }
        }

        impl<const U: Unit> Mul<$t<U>> for $num {
            type Output = $t<U>;

            /// Multiply the value of this unit with a number.
            fn mul(self, rhs: $t<U>) -> Self::Output {
                $t {
                    value: self * rhs.value,
                }
            }
        }

        impl<const L: Unit, const R: Unit> Mul<$t<R>> for $t<L>
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

        impl<const U: Unit> MulAssign<$num> for $t<U> {
            /// Multiply the value of this unit with a number.
            fn mul_assign(&mut self, rhs: $num) {
                self.value *= rhs;
            }
        }

        // ============================
        // Div implementations
        // ============================

        use std::ops::{Div, DivAssign};

        impl<const U: Unit> Div<$num> for $t<U> {
            type Output = Self;

            /// Divides the value of this unit with a number.
            fn div(self, rhs: $num) -> Self::Output {
                Self {
                    value: self.value / rhs,
                }
            }
        }

        impl<const L: Unit, const R: Unit> Div<$t<R>> for $t<L>
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

        impl<const U: Unit> Div<$t<U>> for $num
        where
            $t<{ U.neg() }>: ,
        {
            type Output = $t<{ U.neg() }>;

            fn div(self, rhs: $t<U>) -> Self::Output {
                $t {
                    value: self / rhs.value,
                }
            }
        }

        impl<const U: Unit> DivAssign<$num> for $t<U> {
            /// Divides the value of this unit with a number.
            fn div_assign(&mut self, rhs: $num) {
                self.value *= rhs;
            }
        }
    };
}

quantity_impl!(f64, Quantity);
