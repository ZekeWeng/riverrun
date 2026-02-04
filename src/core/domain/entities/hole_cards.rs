//! Hole cards representation for Texas Hold'em.

use super::card::Card;

/// A player's two private hole cards.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HoleCards {
    cards: [Card; 2],
}

/// HoleCards - Constructors
impl HoleCards {
    /// Create new hole cards from two cards.
    pub fn new(first: Card, second: Card) -> Self {
        HoleCards {
            cards: [first, second],
        }
    }
}

/// HoleCards - Accessors
impl HoleCards {
    /// Get the first card.
    pub fn first(&self) -> Card {
        self.cards[0]
    }

    /// Get the second card.
    pub fn second(&self) -> Card {
        self.cards[1]
    }

    /// Get both cards as a slice.
    pub fn cards(&self) -> &[Card; 2] {
        &self.cards
    }

    /// Check if hole cards are suited (same suit).
    pub fn is_suited(&self) -> bool {
        self.cards[0].same_suit(&self.cards[1])
    }

    /// Check if hole cards are a pocket pair (same rank).
    pub fn is_pair(&self) -> bool {
        self.cards[0].same_rank(&self.cards[1])
    }

    /// Get the gap between cards (0 = connected, 1 = one-gapper, etc).
    pub fn gap(&self) -> u8 {
        let combined = self.cards[0].rank_bits() | self.cards[1].rank_bits();
        let span = (31 - combined.leading_zeros()) - combined.trailing_zeros();
        span.min(13 - span).saturating_sub(1) as u8
    }

    /// Check if hole cards are connected (consecutive ranks).
    pub fn is_connected(&self) -> bool {
        self.gap() == 0
    }

    /// Combine hole cards with a 5-card board to form 7 cards.
    pub fn combine_with_board(&self, board: [Card; 5]) -> [Card; 7] {
        [
            self.cards[0],
            self.cards[1],
            board[0],
            board[1],
            board[2],
            board[3],
            board[4],
        ]
    }
}

impl std::fmt::Display for HoleCards {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.cards[0], self.cards[1])
    }
}

impl From<[Card; 2]> for HoleCards {
    fn from(cards: [Card; 2]) -> Self {
        HoleCards { cards }
    }
}

impl From<(Card, Card)> for HoleCards {
    fn from((first, second): (Card, Card)) -> Self {
        HoleCards::new(first, second)
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
    fn test_new_hole_cards() {
        let hole = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
        );
        assert_eq!(hole.first(), card(Rank::Ace, Suit::Spades));
        assert_eq!(hole.second(), card(Rank::King, Suit::Spades));
    }

    #[test]
    fn test_is_suited() {
        let suited = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
        );
        let offsuit = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
        );
        assert!(suited.is_suited());
        assert!(!offsuit.is_suited());
    }

    #[test]
    fn test_is_pair() {
        let pair = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
        );
        let not_pair = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
        );
        assert!(pair.is_pair());
        assert!(!not_pair.is_pair());
    }

    #[test]
    fn test_is_connected() {
        let connected = HoleCards::new(
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Hearts),
        );
        let ace_two = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::Two, Suit::Hearts),
        );
        let gapped = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::Jack, Suit::Hearts),
        );
        assert!(connected.is_connected());
        assert!(ace_two.is_connected());
        assert!(!gapped.is_connected());
    }

    #[test]
    fn test_gap() {
        let connected = HoleCards::new(
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Hearts),
        );
        let one_gap = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::Queen, Suit::Hearts),
        );
        let two_gap = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::Jack, Suit::Hearts),
        );
        assert_eq!(connected.gap(), 0);
        assert_eq!(one_gap.gap(), 1);
        assert_eq!(two_gap.gap(), 2);
    }

    #[test]
    fn test_display() {
        let hole = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
        );
        assert_eq!(hole.to_string(), "AsKh");
    }

    #[test]
    fn test_from_array() {
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
        ];
        let hole: HoleCards = cards.into();
        assert_eq!(hole.first(), cards[0]);
    }

    #[test]
    fn test_from_tuple() {
        let hole: HoleCards = (
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
        ).into();
        assert_eq!(hole.to_string(), "AsKh");
    }
}
