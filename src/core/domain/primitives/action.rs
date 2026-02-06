//! Player actions in poker.

use super::Chips;
use std::fmt;

/// A player action in a poker hand.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// Fold the hand
    Fold,
    /// Check (pass without betting)
    Check,
    /// Call the current bet
    Call(Chips),
    /// Bet (when no prior bet exists)
    Bet(Chips),
    /// Raise to a specific amount
    Raise(Chips),
    /// Go all-in
    AllIn(Chips),
}

impl Action {
    /// Returns the chip amount involved in this action.
    #[must_use]
    pub const fn amount(self) -> Chips {
        match self {
            Self::Fold | Self::Check => Chips::ZERO,
            Self::Call(chips) | Self::Bet(chips) | Self::Raise(chips) | Self::AllIn(chips) => chips,
        }
    }

    /// Returns whether this action is aggressive (bet/raise).
    #[must_use]
    pub const fn is_aggressive(self) -> bool {
        matches!(self, Self::Bet(_) | Self::Raise(_))
    }

    /// Returns whether this action puts chips in the pot.
    #[must_use]
    pub const fn puts_chips_in(self) -> bool {
        matches!(
            self,
            Self::Call(_) | Self::Bet(_) | Self::Raise(_) | Self::AllIn(_)
        )
    }

    /// Returns whether this action ends the player's participation in the hand.
    #[must_use]
    pub const fn ends_participation(self) -> bool {
        matches!(self, Self::Fold)
    }

    /// Returns the action name as a string.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Fold => "Fold",
            Self::Check => "Check",
            Self::Call(_) => "Call",
            Self::Bet(_) => "Bet",
            Self::Raise(_) => "Raise",
            Self::AllIn(_) => "All-In",
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Fold => write!(f, "Fold"),
            Self::Check => write!(f, "Check"),
            Self::Call(chips) => write!(f, "Call {chips}"),
            Self::Bet(chips) => write!(f, "Bet {chips}"),
            Self::Raise(chips) => write!(f, "Raise to {chips}"),
            Self::AllIn(chips) => write!(f, "All-In {chips}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amount() {
        assert_eq!(Action::Fold.amount(), Chips::ZERO);
        assert_eq!(Action::Check.amount(), Chips::ZERO);
        assert_eq!(Action::Call(Chips::new(100)).amount(), Chips::new(100));
        assert_eq!(Action::Bet(Chips::new(50)).amount(), Chips::new(50));
        assert_eq!(Action::Raise(Chips::new(200)).amount(), Chips::new(200));
        assert_eq!(Action::AllIn(Chips::new(1000)).amount(), Chips::new(1000));
    }

    #[test]
    fn test_is_aggressive() {
        assert!(!Action::Fold.is_aggressive());
        assert!(!Action::Check.is_aggressive());
        assert!(!Action::Call(Chips::new(100)).is_aggressive());
        assert!(Action::Bet(Chips::new(50)).is_aggressive());
        assert!(Action::Raise(Chips::new(200)).is_aggressive());
        assert!(!Action::AllIn(Chips::new(1000)).is_aggressive());
    }

    #[test]
    fn test_puts_chips_in() {
        assert!(!Action::Fold.puts_chips_in());
        assert!(!Action::Check.puts_chips_in());
        assert!(Action::Call(Chips::new(100)).puts_chips_in());
        assert!(Action::Bet(Chips::new(50)).puts_chips_in());
        assert!(Action::Raise(Chips::new(200)).puts_chips_in());
        assert!(Action::AllIn(Chips::new(1000)).puts_chips_in());
    }

    #[test]
    fn test_ends_participation() {
        assert!(Action::Fold.ends_participation());
        assert!(!Action::Check.ends_participation());
        assert!(!Action::Call(Chips::new(100)).ends_participation());
    }

    #[test]
    fn test_name() {
        assert_eq!(Action::Fold.name(), "Fold");
        assert_eq!(Action::Check.name(), "Check");
        assert_eq!(Action::Call(Chips::new(100)).name(), "Call");
        assert_eq!(Action::Bet(Chips::new(50)).name(), "Bet");
        assert_eq!(Action::Raise(Chips::new(200)).name(), "Raise");
        assert_eq!(Action::AllIn(Chips::new(1000)).name(), "All-In");
    }

    #[test]
    fn test_display() {
        assert_eq!(Action::Fold.to_string(), "Fold");
        assert_eq!(Action::Check.to_string(), "Check");
        assert_eq!(Action::Call(Chips::new(100)).to_string(), "Call 100");
        assert_eq!(Action::Bet(Chips::new(50)).to_string(), "Bet 50");
        assert_eq!(Action::Raise(Chips::new(200)).to_string(), "Raise to 200");
        assert_eq!(Action::AllIn(Chips::new(1000)).to_string(), "All-In 1000");
    }
}
