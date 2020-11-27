//! All SI base units and more constants.

#![allow(non_upper_case_globals)]

use crate::Unit;

const NONE: Unit = Unit {
    A: 0,
    K: 0,
    cd: 0,
    kg: 0,
    m: 0,
    mol: 0,
    s: 0,
};

/// Time in seconds
pub const s: Unit = Unit { s: 1, ..NONE };

/// Length in metre
pub const m: Unit = Unit { m: 1, ..NONE };

/// Mass in kilogram
pub const kg: Unit = Unit { kg: 1, ..NONE };

/// Electric current in ampere
pub const A: Unit = Unit { A: 1, ..NONE };

/// Temperature in kelvin
pub const K: Unit = Unit { K: 1, ..NONE };

/// Amount of substance in mole
pub const mol: Unit = Unit { mol: 1, ..NONE };

/// Luminous intensity in candela
pub const cd: Unit = Unit { cd: 1, ..NONE };
