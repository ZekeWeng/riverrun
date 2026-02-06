//! Street/stage of a poker hand.

use std::fmt;

/// The current street/stage of the hand.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Street {
    Preflop = 0,
    Flop = 1,
    Turn = 2,
    River = 3,
}

impl Street {
    /// Returns the number of community cards for this street.
    #[must_use]
    pub const fn card_count(self) -> usize {
        match self {
            Self::Preflop => 0,
            Self::Flop => 3,
            Self::Turn => 4,
            Self::River => 5,
        }
    }

    /// Returns the next street, if any.
    #[must_use]
    pub const fn next(self) -> Option<Self> {
        match self {
            Self::Preflop => Some(Self::Flop),
            Self::Flop => Some(Self::Turn),
            Self::Turn => Some(Self::River),
            Self::River => None,
        }
    }

    /// Returns whether this is the final street.
    #[must_use]
    pub const fn is_final(self) -> bool {
        matches!(self, Self::River)
    }

    /// Creates a Street from the number of community cards.
    #[must_use]
    pub const fn from_card_count(count: usize) -> Option<Self> {
        match count {
            0 => Some(Self::Preflop),
            3 => Some(Self::Flop),
            4 => Some(Self::Turn),
            5 => Some(Self::River),
            _ => None,
        }
    }
}

impl fmt::Display for Street {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Preflop => write!(f, "Preflop"),
            Self::Flop => write!(f, "Flop"),
            Self::Turn => write!(f, "Turn"),
            Self::River => write!(f, "River"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_count() {
        assert_eq!(Street::Preflop.card_count(), 0);
        assert_eq!(Street::Flop.card_count(), 3);
        assert_eq!(Street::Turn.card_count(), 4);
        assert_eq!(Street::River.card_count(), 5);
    }

    #[test]
    fn test_next() {
        assert_eq!(Street::Preflop.next(), Some(Street::Flop));
        assert_eq!(Street::Flop.next(), Some(Street::Turn));
        assert_eq!(Street::Turn.next(), Some(Street::River));
        assert_eq!(Street::River.next(), None);
    }

    #[test]
    fn test_is_final() {
        assert!(!Street::Preflop.is_final());
        assert!(!Street::Flop.is_final());
        assert!(!Street::Turn.is_final());
        assert!(Street::River.is_final());
    }

    #[test]
    fn test_from_card_count() {
        assert_eq!(Street::from_card_count(0), Some(Street::Preflop));
        assert_eq!(Street::from_card_count(3), Some(Street::Flop));
        assert_eq!(Street::from_card_count(4), Some(Street::Turn));
        assert_eq!(Street::from_card_count(5), Some(Street::River));
        assert_eq!(Street::from_card_count(1), None);
        assert_eq!(Street::from_card_count(2), None);
    }

    #[test]
    fn test_display() {
        assert_eq!(Street::Preflop.to_string(), "Preflop");
        assert_eq!(Street::Flop.to_string(), "Flop");
        assert_eq!(Street::Turn.to_string(), "Turn");
        assert_eq!(Street::River.to_string(), "River");
    }

    #[test]
    fn test_ordering() {
        assert!(Street::Preflop < Street::Flop);
        assert!(Street::Flop < Street::Turn);
        assert!(Street::Turn < Street::River);
    }
}
