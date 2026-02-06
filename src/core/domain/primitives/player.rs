//! Player identification and positioning.

use std::fmt;

/// Unique identifier for a player in a game.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlayerId(pub u8);

impl PlayerId {
    /// Creates a new player ID.
    #[must_use]
    pub const fn new(id: u8) -> Self {
        Self(id)
    }

    /// Returns the underlying player ID value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self.0
    }

    /// Returns the player ID as a usize for indexing.
    #[must_use]
    pub const fn as_index(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player {}", self.0)
    }
}

impl From<u8> for PlayerId {
    fn from(id: u8) -> Self {
        Self::new(id)
    }
}

impl From<usize> for PlayerId {
    #[allow(clippy::cast_possible_truncation)]
    fn from(id: usize) -> Self {
        Self::new(id as u8)
    }
}

/// Table position in poker.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Position {
    /// Button (dealer position)
    Button,
    /// Small blind
    SmallBlind,
    /// Big blind
    BigBlind,
    /// Under the gun (first to act preflop)
    UTG,
    /// Under the gun + 1
    UTG1,
    /// Under the gun + 2
    UTG2,
    /// Middle position
    MP,
    /// Middle position + 1
    MP1,
    /// Hijack (two seats before button)
    Hijack,
    /// Cutoff (one seat before button)
    Cutoff,
}

impl Position {
    /// Returns the standard abbreviation for this position.
    #[must_use]
    pub const fn abbrev(self) -> &'static str {
        match self {
            Self::Button => "BTN",
            Self::SmallBlind => "SB",
            Self::BigBlind => "BB",
            Self::UTG => "UTG",
            Self::UTG1 => "UTG+1",
            Self::UTG2 => "UTG+2",
            Self::MP => "MP",
            Self::MP1 => "MP+1",
            Self::Hijack => "HJ",
            Self::Cutoff => "CO",
        }
    }

    /// Returns whether this position is in the blinds.
    #[must_use]
    pub const fn is_blind(self) -> bool {
        matches!(self, Self::SmallBlind | Self::BigBlind)
    }

    /// Returns whether this position is considered late position.
    #[must_use]
    pub const fn is_late(self) -> bool {
        matches!(self, Self::Button | Self::Cutoff | Self::Hijack)
    }

    /// Returns whether this position is considered early position.
    #[must_use]
    pub const fn is_early(self) -> bool {
        matches!(self, Self::UTG | Self::UTG1 | Self::UTG2)
    }

    /// Returns positions for a given table size (2-10 players).
    #[must_use]
    pub fn for_table_size(players: usize) -> Option<Vec<Self>> {
        if !(2..=10).contains(&players) {
            return None;
        }

        let positions = match players {
            2 => vec![Self::Button, Self::BigBlind],
            3 => vec![Self::Button, Self::SmallBlind, Self::BigBlind],
            4 => vec![Self::Button, Self::SmallBlind, Self::BigBlind, Self::UTG],
            5 => vec![
                Self::Button,
                Self::SmallBlind,
                Self::BigBlind,
                Self::UTG,
                Self::Cutoff,
            ],
            6 => vec![
                Self::Button,
                Self::SmallBlind,
                Self::BigBlind,
                Self::UTG,
                Self::Hijack,
                Self::Cutoff,
            ],
            7 => vec![
                Self::Button,
                Self::SmallBlind,
                Self::BigBlind,
                Self::UTG,
                Self::MP,
                Self::Hijack,
                Self::Cutoff,
            ],
            8 => vec![
                Self::Button,
                Self::SmallBlind,
                Self::BigBlind,
                Self::UTG,
                Self::UTG1,
                Self::MP,
                Self::Hijack,
                Self::Cutoff,
            ],
            9 => vec![
                Self::Button,
                Self::SmallBlind,
                Self::BigBlind,
                Self::UTG,
                Self::UTG1,
                Self::MP,
                Self::MP1,
                Self::Hijack,
                Self::Cutoff,
            ],
            10 => vec![
                Self::Button,
                Self::SmallBlind,
                Self::BigBlind,
                Self::UTG,
                Self::UTG1,
                Self::UTG2,
                Self::MP,
                Self::MP1,
                Self::Hijack,
                Self::Cutoff,
            ],
            _ => return None,
        };
        Some(positions)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.abbrev())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_id() {
        let player = PlayerId::new(5);
        assert_eq!(player.value(), 5);
        assert_eq!(player.as_index(), 5);
        assert_eq!(player.to_string(), "Player 5");
    }

    #[test]
    fn test_player_id_from() {
        let p1: PlayerId = 3u8.into();
        assert_eq!(p1.value(), 3);

        let p2: PlayerId = 7usize.into();
        assert_eq!(p2.value(), 7);
    }

    #[test]
    fn test_position_abbrev() {
        assert_eq!(Position::Button.abbrev(), "BTN");
        assert_eq!(Position::SmallBlind.abbrev(), "SB");
        assert_eq!(Position::BigBlind.abbrev(), "BB");
        assert_eq!(Position::UTG.abbrev(), "UTG");
        assert_eq!(Position::Cutoff.abbrev(), "CO");
        assert_eq!(Position::Hijack.abbrev(), "HJ");
    }

    #[test]
    fn test_position_predicates() {
        assert!(Position::SmallBlind.is_blind());
        assert!(Position::BigBlind.is_blind());
        assert!(!Position::Button.is_blind());

        assert!(Position::Button.is_late());
        assert!(Position::Cutoff.is_late());
        assert!(!Position::UTG.is_late());

        assert!(Position::UTG.is_early());
        assert!(Position::UTG1.is_early());
        assert!(!Position::Button.is_early());
    }

    #[test]
    fn test_for_table_size() {
        assert!(Position::for_table_size(1).is_none());
        assert!(Position::for_table_size(11).is_none());

        let heads_up = Position::for_table_size(2).unwrap();
        assert_eq!(heads_up.len(), 2);

        let six_max = Position::for_table_size(6).unwrap();
        assert_eq!(six_max.len(), 6);

        let full_ring = Position::for_table_size(10).unwrap();
        assert_eq!(full_ring.len(), 10);
    }
}
