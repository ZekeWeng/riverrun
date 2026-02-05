//! Evaluated poker hand representation.

use std::cmp::Ordering;
use std::fmt;

use super::card::Card;

/// Poker hand category.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum HandRank {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    Straight = 4,
    Flush = 5,
    FullHouse = 6,
    FourOfAKind = 7,
    StraightFlush = 8,
}

/// `HandRank` - Constructors
impl HandRank {
    /// Create a `HandRank` from a numeric strength (1-7462).
    #[must_use]
    pub const fn from_strength(strength: u16) -> Self {
        match strength {
            1..=10 => Self::StraightFlush,
            11..=166 => Self::FourOfAKind,
            167..=322 => Self::FullHouse,
            323..=1599 => Self::Flush,
            1600..=1609 => Self::Straight,
            1610..=2467 => Self::ThreeOfAKind,
            2468..=3325 => Self::TwoPair,
            3326..=6185 => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

/// `HandRank` - Accessors
impl HandRank {
    /// Get the display name of the hand rank.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::HighCard => "High Card",
            Self::OnePair => "One Pair",
            Self::TwoPair => "Two Pair",
            Self::ThreeOfAKind => "Three of a Kind",
            Self::Straight => "Straight",
            Self::Flush => "Flush",
            Self::FullHouse => "Full House",
            Self::FourOfAKind => "Four of a Kind",
            Self::StraightFlush => "Straight Flush",
        }
    }
}

impl fmt::Display for HandRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// An evaluated poker hand.
///
/// Represents the best 5-card hand with its category and strength.
/// Lower strength values indicate stronger hands (1 = royal flush).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
    rank: HandRank,
    strength: u16,
}

/// Hand - Constructors
impl Hand {
    /// Create a new hand from cards and strength.
    #[must_use]
    pub const fn new(cards: [Card; 5], strength: u16) -> Self {
        let rank = HandRank::from_strength(strength);
        Self {
            cards,
            rank,
            strength,
        }
    }
}

/// Hand - Accessors
impl Hand {
    /// Get the 5 cards that form this hand.
    #[must_use]
    pub const fn cards(&self) -> &[Card; 5] {
        &self.cards
    }

    /// Get a specific card by index (0-4).
    #[must_use]
    pub const fn card(&self, index: usize) -> Card {
        self.cards[index]
    }

    /// Get the hand category (e.g., Flush, Full House).
    #[must_use]
    pub const fn rank(&self) -> HandRank {
        self.rank
    }

    /// Get the numeric strength (1 = best, 7462 = worst).
    #[must_use]
    pub const fn strength(&self) -> u16 {
        self.strength
    }

    /// Check if this hand is a specific rank.
    #[must_use]
    pub const fn is_rank(&self, rank: HandRank) -> bool {
        matches!(
            (&self.rank, &rank),
            (HandRank::HighCard, HandRank::HighCard)
                | (HandRank::OnePair, HandRank::OnePair)
                | (HandRank::TwoPair, HandRank::TwoPair)
                | (HandRank::ThreeOfAKind, HandRank::ThreeOfAKind)
                | (HandRank::Straight, HandRank::Straight)
                | (HandRank::Flush, HandRank::Flush)
                | (HandRank::FullHouse, HandRank::FullHouse)
                | (HandRank::FourOfAKind, HandRank::FourOfAKind)
                | (HandRank::StraightFlush, HandRank::StraightFlush)
        )
    }

    /// Check if this hand is a straight flush (includes royal flush).
    #[must_use]
    pub const fn is_straight_flush(&self) -> bool {
        matches!(self.rank, HandRank::StraightFlush)
    }

    /// Check if this hand is a royal flush (best possible hand).
    #[must_use]
    pub const fn is_royal_flush(&self) -> bool {
        self.strength == 1
    }

    /// Check if this hand is four of a kind.
    #[must_use]
    pub const fn is_four_of_a_kind(&self) -> bool {
        matches!(self.rank, HandRank::FourOfAKind)
    }

    /// Check if this hand is a full house.
    #[must_use]
    pub const fn is_full_house(&self) -> bool {
        matches!(self.rank, HandRank::FullHouse)
    }

    /// Check if this hand is a flush.
    #[must_use]
    pub const fn is_flush(&self) -> bool {
        matches!(self.rank, HandRank::Flush)
    }

    /// Check if this hand is a straight.
    #[must_use]
    pub const fn is_straight(&self) -> bool {
        matches!(self.rank, HandRank::Straight)
    }

    /// Check if this hand is three of a kind.
    #[must_use]
    pub const fn is_three_of_a_kind(&self) -> bool {
        matches!(self.rank, HandRank::ThreeOfAKind)
    }

    /// Check if this hand is two pair.
    #[must_use]
    pub const fn is_two_pair(&self) -> bool {
        matches!(self.rank, HandRank::TwoPair)
    }

    /// Check if this hand is one pair.
    #[must_use]
    pub const fn is_one_pair(&self) -> bool {
        matches!(self.rank, HandRank::OnePair)
    }

    /// Check if this hand is high card only.
    #[must_use]
    pub const fn is_high_card(&self) -> bool {
        matches!(self.rank, HandRank::HighCard)
    }
}

/// Hand - Operations
impl Hand {
    /// Check if this hand beats another hand.
    #[must_use]
    pub const fn beats(&self, other: &Self) -> bool {
        self.strength < other.strength
    }

    /// Check if this hand ties with another hand.
    #[must_use]
    pub const fn ties(&self, other: &Self) -> bool {
        self.strength == other.strength
    }

    /// Check if this hand loses to another hand.
    #[must_use]
    pub const fn loses_to(&self, other: &Self) -> bool {
        self.strength > other.strength
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // Lower strength is better, so reverse the comparison
        other.strength.cmp(&self.strength)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [{}{}{}{}{}]",
            self.rank,
            self.cards[0],
            self.cards[1],
            self.cards[2],
            self.cards[3],
            self.cards[4]
        )
    }
}

impl From<([Card; 5], u16)> for Hand {
    fn from((cards, strength): ([Card; 5], u16)) -> Self {
        Self::new(cards, strength)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::entities::card::{Rank, Suit};

    fn card(rank: Rank, suit: Suit) -> Card {
        Card::new(rank, suit)
    }

    #[test]
    fn test_hand_rank_from_strength() {
        assert_eq!(HandRank::from_strength(1), HandRank::StraightFlush);
        assert_eq!(HandRank::from_strength(10), HandRank::StraightFlush);
        assert_eq!(HandRank::from_strength(11), HandRank::FourOfAKind);
        assert_eq!(HandRank::from_strength(166), HandRank::FourOfAKind);
        assert_eq!(HandRank::from_strength(167), HandRank::FullHouse);
        assert_eq!(HandRank::from_strength(322), HandRank::FullHouse);
        assert_eq!(HandRank::from_strength(323), HandRank::Flush);
        assert_eq!(HandRank::from_strength(1599), HandRank::Flush);
        assert_eq!(HandRank::from_strength(1600), HandRank::Straight);
        assert_eq!(HandRank::from_strength(1609), HandRank::Straight);
        assert_eq!(HandRank::from_strength(1610), HandRank::ThreeOfAKind);
        assert_eq!(HandRank::from_strength(2467), HandRank::ThreeOfAKind);
        assert_eq!(HandRank::from_strength(2468), HandRank::TwoPair);
        assert_eq!(HandRank::from_strength(3325), HandRank::TwoPair);
        assert_eq!(HandRank::from_strength(3326), HandRank::OnePair);
        assert_eq!(HandRank::from_strength(6185), HandRank::OnePair);
        assert_eq!(HandRank::from_strength(6186), HandRank::HighCard);
        assert_eq!(HandRank::from_strength(7462), HandRank::HighCard);
    }

    #[test]
    fn test_hand_rank_ordering() {
        assert!(HandRank::StraightFlush > HandRank::FourOfAKind);
        assert!(HandRank::FourOfAKind > HandRank::FullHouse);
        assert!(HandRank::FullHouse > HandRank::Flush);
        assert!(HandRank::Flush > HandRank::Straight);
        assert!(HandRank::Straight > HandRank::ThreeOfAKind);
        assert!(HandRank::ThreeOfAKind > HandRank::TwoPair);
        assert!(HandRank::TwoPair > HandRank::OnePair);
        assert!(HandRank::OnePair > HandRank::HighCard);
    }

    #[test]
    fn test_hand_creation() {
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Spades),
        ];
        let hand = Hand::new(cards, 1);

        assert_eq!(hand.rank(), HandRank::StraightFlush);
        assert_eq!(hand.strength(), 1);
        assert!(hand.is_royal_flush());
        assert!(hand.is_straight_flush());
    }

    #[test]
    fn test_hand_card_accessor() {
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Spades),
        ];
        let hand = Hand::new(cards, 1);

        assert_eq!(hand.card(0), cards[0]);
        assert_eq!(hand.card(4), cards[4]);
        assert_eq!(hand.cards(), &cards);
    }

    #[test]
    fn test_hand_comparison() {
        let cards1 = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Spades),
        ];
        let royal_flush = Hand::new(cards1, 1);

        let cards2 = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::Ace, Suit::Clubs),
            card(Rank::King, Suit::Spades),
        ];
        let four_aces = Hand::new(cards2, 11);

        assert!(royal_flush.beats(&four_aces));
        assert!(four_aces.loses_to(&royal_flush));
        assert!(!royal_flush.ties(&four_aces));
        assert!(royal_flush > four_aces);
    }

    #[test]
    fn test_hand_ties() {
        let cards1 = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Spades),
        ];
        let hand1 = Hand::new(cards1, 1);

        let cards2 = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Hearts),
            card(Rank::Jack, Suit::Hearts),
            card(Rank::Ten, Suit::Hearts),
        ];
        let hand2 = Hand::new(cards2, 1);

        assert!(hand1.ties(&hand2));
        assert!(!hand1.beats(&hand2));
        assert!(!hand1.loses_to(&hand2));
    }

    #[test]
    fn test_hand_display() {
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Spades),
        ];
        let hand = Hand::new(cards, 1);
        let display = format!("{}", hand);
        assert!(display.contains("Straight Flush"));
        assert!(display.contains("As"));
    }

    #[test]
    fn test_hand_from_tuple() {
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Spades),
        ];
        let hand: Hand = (cards, 1).into();
        assert_eq!(hand.strength(), 1);
        assert!(hand.is_royal_flush());
    }

    #[test]
    fn test_hand_rank_predicates() {
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Diamonds),
            card(Rank::Jack, Suit::Clubs),
            card(Rank::Nine, Suit::Spades),
        ];

        let high_card = Hand::new(cards, 7000);
        assert!(high_card.is_high_card());
        assert!(!high_card.is_one_pair());

        let one_pair = Hand::new(cards, 4000);
        assert!(one_pair.is_one_pair());
        assert!(!one_pair.is_two_pair());

        let two_pair = Hand::new(cards, 3000);
        assert!(two_pair.is_two_pair());

        let three_kind = Hand::new(cards, 2000);
        assert!(three_kind.is_three_of_a_kind());

        let straight = Hand::new(cards, 1605);
        assert!(straight.is_straight());

        let flush = Hand::new(cards, 500);
        assert!(flush.is_flush());

        let full_house = Hand::new(cards, 200);
        assert!(full_house.is_full_house());

        let four_kind = Hand::new(cards, 50);
        assert!(four_kind.is_four_of_a_kind());

        let straight_flush = Hand::new(cards, 5);
        assert!(straight_flush.is_straight_flush());
        assert!(!straight_flush.is_royal_flush());
    }
}
