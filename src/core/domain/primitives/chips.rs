//! Chip amount representation.

use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// Represents a chip amount in a poker game.
///
/// Uses u64 internally to support large tournament stacks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Chips(pub u64);

impl Chips {
    /// Zero chips.
    pub const ZERO: Self = Self(0);

    /// Creates a new chip amount.
    #[must_use]
    pub const fn new(amount: u64) -> Self {
        Self(amount)
    }

    /// Returns the underlying chip value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns whether this represents zero chips.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Saturating subtraction. Returns zero if result would be negative.
    #[must_use]
    pub const fn saturating_sub(self, other: Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }

    /// Returns the minimum of two chip amounts.
    #[must_use]
    pub const fn min(self, other: Self) -> Self {
        if self.0 < other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the maximum of two chip amounts.
    #[must_use]
    pub const fn max(self, other: Self) -> Self {
        if self.0 > other.0 {
            self
        } else {
            other
        }
    }
}

impl fmt::Display for Chips {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for Chips {
    fn from(amount: u64) -> Self {
        Self::new(amount)
    }
}

impl From<u32> for Chips {
    fn from(amount: u32) -> Self {
        Self::new(u64::from(amount))
    }
}

impl From<Chips> for u64 {
    fn from(chips: Chips) -> Self {
        chips.0
    }
}

impl Add for Chips {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl AddAssign for Chips {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Sub for Chips {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl SubAssign for Chips {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_value() {
        let chips = Chips::new(1000);
        assert_eq!(chips.value(), 1000);
    }

    #[test]
    fn test_zero() {
        assert!(Chips::ZERO.is_zero());
        assert!(!Chips::new(1).is_zero());
    }

    #[test]
    fn test_arithmetic() {
        let a = Chips::new(100);
        let b = Chips::new(50);

        assert_eq!(a + b, Chips::new(150));
        assert_eq!(a - b, Chips::new(50));
    }

    #[test]
    fn test_add_assign() {
        let mut chips = Chips::new(100);
        chips += Chips::new(50);
        assert_eq!(chips, Chips::new(150));
    }

    #[test]
    fn test_sub_assign() {
        let mut chips = Chips::new(100);
        chips -= Chips::new(30);
        assert_eq!(chips, Chips::new(70));
    }

    #[test]
    fn test_saturating_sub() {
        let chips = Chips::new(100);
        assert_eq!(chips.saturating_sub(Chips::new(50)), Chips::new(50));
        assert_eq!(chips.saturating_sub(Chips::new(150)), Chips::ZERO);
    }

    #[test]
    fn test_min_max() {
        let a = Chips::new(100);
        let b = Chips::new(200);

        assert_eq!(a.min(b), Chips::new(100));
        assert_eq!(a.max(b), Chips::new(200));
    }

    #[test]
    fn test_from() {
        let from_u64: Chips = 1000u64.into();
        assert_eq!(from_u64, Chips::new(1000));

        let from_u32: Chips = 500u32.into();
        assert_eq!(from_u32, Chips::new(500));

        let to_u64: u64 = Chips::new(250).into();
        assert_eq!(to_u64, 250);
    }

    #[test]
    fn test_ordering() {
        assert!(Chips::new(100) < Chips::new(200));
        assert!(Chips::new(200) > Chips::new(100));
        assert!(Chips::new(100) == Chips::new(100));
    }

    #[test]
    fn test_display() {
        assert_eq!(Chips::new(1000).to_string(), "1000");
    }
}
