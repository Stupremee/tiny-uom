//! Constants that represent a quantity of `1` of
//! different units.

#![allow(non_upper_case_globals)]

use crate::{units, Quantity};

/// Time in seconds
pub const s: Quantity<{ units::s }> = Quantity::new(1.0);

/// Length in metre
pub const m: Quantity<{ units::m }> = Quantity::new(1.0);

/// Mass in kilogram
pub const kg: Quantity<{ units::kg }> = Quantity::new(1.0);

/// Electric current in ampere
pub const A: Quantity<{ units::A }> = Quantity::new(1.0);

/// Temperature in kelvin
pub const K: Quantity<{ units::K }> = Quantity::new(1.0);

/// Amount of substance in mole
pub const mol: Quantity<{ units::mol }> = Quantity::new(1.0);

/// Luminous intensity in candela
pub const cd: Quantity<{ units::cd }> = Quantity::new(1.0);
