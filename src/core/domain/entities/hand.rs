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

/// HandRank - Constructors
impl HandRank {
    /// Create a HandRank from a numeric strength (1-7462).
    pub fn from_strength(strength: u16) -> Self {
        match strength {
            1..=10 => HandRank::StraightFlush,
            11..=166 => HandRank::FourOfAKind,
            167..=322 => HandRank::FullHouse,
            323..=1599 => HandRank::Flush,
            1600..=1609 => HandRank::Straight,
            1610..=2467 => HandRank::ThreeOfAKind,
            2468..=3325 => HandRank::TwoPair,
            3326..=6185 => HandRank::OnePair,
            _ => HandRank::HighCard,
        }
    }
}

/// HandRank - Accessors
impl HandRank {
    /// Get the display name of the hand rank.
    pub fn name(&self) -> &'static str {
        match self {
            HandRank::HighCard => "High Card",
            HandRank::OnePair => "One Pair",
            HandRank::TwoPair => "Two Pair",
            HandRank::ThreeOfAKind => "Three of a Kind",
            HandRank::Straight => "Straight",
            HandRank::Flush => "Flush",
            HandRank::FullHouse => "Full House",
            HandRank::FourOfAKind => "Four of a Kind",
            HandRank::StraightFlush => "Straight Flush",
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
    pub fn new(cards: [Card; 5], strength: u16) -> Self {
        let rank = HandRank::from_strength(strength);
        Hand {
            cards,
            rank,
            strength,
        }
    }
}

/// Hand - Accessors
impl Hand {
    /// Get the 5 cards that form this hand.
    pub fn cards(&self) -> &[Card; 5] {
        &self.cards
    }

    /// Get a specific card by index (0-4).
    pub fn card(&self, index: usize) -> Card {
        self.cards[index]
    }

    /// Get the hand category (e.g., Flush, Full House).
    pub fn rank(&self) -> HandRank {
        self.rank
    }

    /// Get the numeric strength (1 = best, 7462 = worst).
    pub fn strength(&self) -> u16 {
        self.strength
    }

    /// Check if this hand is a specific rank.
    pub fn is_rank(&self, rank: HandRank) -> bool {
        self.rank == rank
    }

    /// Check if this hand is a straight flush (includes royal flush).
    pub fn is_straight_flush(&self) -> bool {
        self.rank == HandRank::StraightFlush
    }

    /// Check if this hand is a royal flush (best possible hand).
    pub fn is_royal_flush(&self) -> bool {
        self.strength == 1
    }

    /// Check if this hand is four of a kind.
    pub fn is_four_of_a_kind(&self) -> bool {
        self.rank == HandRank::FourOfAKind
    }

    /// Check if this hand is a full house.
    pub fn is_full_house(&self) -> bool {
        self.rank == HandRank::FullHouse
    }

    /// Check if this hand is a flush.
    pub fn is_flush(&self) -> bool {
        self.rank == HandRank::Flush
    }

    /// Check if this hand is a straight.
    pub fn is_straight(&self) -> bool {
        self.rank == HandRank::Straight
    }

    /// Check if this hand is three of a kind.
    pub fn is_three_of_a_kind(&self) -> bool {
        self.rank == HandRank::ThreeOfAKind
    }

    /// Check if this hand is two pair.
    pub fn is_two_pair(&self) -> bool {
        self.rank == HandRank::TwoPair
    }

    /// Check if this hand is one pair.
    pub fn is_one_pair(&self) -> bool {
        self.rank == HandRank::OnePair
    }

    /// Check if this hand is high card only.
    pub fn is_high_card(&self) -> bool {
        self.rank == HandRank::HighCard
    }
}

/// Hand - Operations
impl Hand {
    /// Check if this hand beats another hand.
    pub fn beats(&self, other: &Hand) -> bool {
        self.strength < other.strength
    }

    /// Check if this hand ties with another hand.
    pub fn ties(&self, other: &Hand) -> bool {
        self.strength == other.strength
    }

    /// Check if this hand loses to another hand.
    pub fn loses_to(&self, other: &Hand) -> bool {
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
        Hand::new(cards, strength)
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
