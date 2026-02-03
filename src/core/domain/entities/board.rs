//! Community board cards for Texas Hold'em.

use super::card::Card;

/// The community board cards (flop, turn, river).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    cards: Vec<Card>,
    street: Street,
}

/// Constructors
impl Board {
    /// Create an empty board (preflop).
    pub fn new() -> Self {
        Board {
            cards: Vec::new(),
            street: Street::Preflop,
        }
    }

    /// Create a board with the given cards.
    /// Returns None if invalid card count (must be 0, 3, 4, or 5).
    pub fn with_cards(cards: Vec<Card>) -> Option<Self> {
        let street = match cards.len() {
            0 => Street::Preflop,
            3 => Street::Flop,
            4 => Street::Turn,
            5 => Street::River,
            _ => return None,
        };
        Some(Board { cards, street })
    }
}

/// Accessors
impl Board {
    /// Get all cards on the board.
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    /// Get the number of cards on the board.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Check if the board is empty (preflop).
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Check if the board is complete (river dealt).
    pub fn is_complete(&self) -> bool {
        self.street == Street::River
    }

    /// Get the current street.
    pub fn street(&self) -> Street {
        self.street
    }
}

/// Operations
impl Board {
    /// Deal the flop cards.
    /// Returns false if not at preflop stage.
    pub fn deal_flop(&mut self, c1: Card, c2: Card, c3: Card) -> bool {
        if self.street != Street::Preflop {
            return false;
        }
        self.cards.push(c1);
        self.cards.push(c2);
        self.cards.push(c3);
        self.street = Street::Flop;
        true
    }

    /// Deal the turn card.
    /// Returns false if not at flop stage.
    pub fn deal_turn(&mut self, card: Card) -> bool {
        if self.street != Street::Flop {
            return false;
        }
        self.cards.push(card);
        self.street = Street::Turn;
        true
    }

    /// Deal the river card.
    /// Returns false if not at turn stage.
    pub fn deal_river(&mut self, card: Card) -> bool {
        if self.street != Street::Turn {
            return false;
        }
        self.cards.push(card);
        self.street = Street::River;
        true
    }

    /// Clear the board for a new hand.
    pub fn clear(&mut self) {
        self.cards.clear();
        self.street = Street::Preflop;
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.cards.is_empty() {
            return write!(f, "[]");
        }
        let cards: Vec<String> = self.cards.iter().map(|c| c.to_string()).collect();
        write!(f, "[{}]", cards.join(" "))
    }
}

/// The current street/stage of the hand.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Street {
    Preflop,
    Flop,
    Turn,
    River,
}

impl std::fmt::Display for Street {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Street::Preflop => write!(f, "Preflop"),
            Street::Flop => write!(f, "Flop"),
            Street::Turn => write!(f, "Turn"),
            Street::River => write!(f, "River"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::entities::card::{Rank, Suit};

    fn card(rank: Rank, suit: Suit) -> Card {
        Card::new(rank, suit)
    }

    fn make_flop() -> Board {
        let mut board = Board::new();
        board.deal_flop(
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Diamonds),
        );
        board
    }

    #[test]
    fn test_new_board() {
        let board = Board::new();
        assert!(board.is_empty());
        assert_eq!(board.len(), 0);
        assert_eq!(board.street(), Street::Preflop);
    }

    #[test]
    fn test_deal_flop() {
        let mut board = Board::new();
        assert!(board.deal_flop(
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Diamonds),
        ));
        assert_eq!(board.len(), 3);
        assert_eq!(board.street(), Street::Flop);
    }

    #[test]
    fn test_deal_flop_wrong_stage() {
        let mut board = make_flop();
        assert!(!board.deal_flop(
            card(Rank::Two, Suit::Clubs),
            card(Rank::Three, Suit::Clubs),
            card(Rank::Four, Suit::Clubs),
        ));
    }

    #[test]
    fn test_deal_turn() {
        let mut board = make_flop();
        assert!(board.deal_turn(card(Rank::Jack, Suit::Clubs)));
        assert_eq!(board.len(), 4);
        assert_eq!(board.street(), Street::Turn);
    }

    #[test]
    fn test_deal_river() {
        let mut board = make_flop();
        board.deal_turn(card(Rank::Jack, Suit::Clubs));
        assert!(board.deal_river(card(Rank::Ten, Suit::Spades)));
        assert_eq!(board.len(), 5);
        assert_eq!(board.street(), Street::River);
        assert!(board.is_complete());
    }

    #[test]
    fn test_deal_turn_wrong_stage() {
        let mut board = Board::new();
        assert!(!board.deal_turn(card(Rank::Jack, Suit::Clubs)));
    }

    #[test]
    fn test_deal_river_wrong_stage() {
        let mut board = make_flop();
        assert!(!board.deal_river(card(Rank::Ten, Suit::Spades)));
    }

    #[test]
    fn test_with_cards_flop() {
        let board = Board::with_cards(vec![
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Diamonds),
        ])
        .unwrap();
        assert_eq!(board.len(), 3);
        assert_eq!(board.street(), Street::Flop);
    }

    #[test]
    fn test_with_cards_turn() {
        let board = Board::with_cards(vec![
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Diamonds),
            card(Rank::Jack, Suit::Clubs),
        ])
        .unwrap();
        assert_eq!(board.len(), 4);
        assert_eq!(board.street(), Street::Turn);
    }

    #[test]
    fn test_with_cards_river() {
        let board = Board::with_cards(vec![
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Diamonds),
            card(Rank::Jack, Suit::Clubs),
            card(Rank::Ten, Suit::Spades),
        ])
        .unwrap();
        assert_eq!(board.len(), 5);
        assert_eq!(board.street(), Street::River);
        assert!(board.is_complete());
    }

    #[test]
    fn test_with_cards_invalid_count() {
        // 1 card is invalid
        assert!(Board::with_cards(vec![card(Rank::Ace, Suit::Spades)]).is_none());
        // 2 cards is invalid
        assert!(Board::with_cards(vec![
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
        ])
        .is_none());
        // 6 cards is invalid
        assert!(Board::with_cards(vec![
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Diamonds),
            card(Rank::Jack, Suit::Clubs),
            card(Rank::Ten, Suit::Spades),
            card(Rank::Nine, Suit::Hearts),
        ])
        .is_none());
    }

    #[test]
    fn test_cards_accessor() {
        let board = make_flop();
        let cards = board.cards();
        assert_eq!(cards.len(), 3);
        assert_eq!(cards[0], card(Rank::Ace, Suit::Spades));
        assert_eq!(cards[1], card(Rank::King, Suit::Hearts));
        assert_eq!(cards[2], card(Rank::Queen, Suit::Diamonds));
    }

    #[test]
    fn test_display() {
        let board = make_flop();
        assert_eq!(board.to_string(), "[As Kh Qd]");
    }

    #[test]
    fn test_display_empty() {
        let board = Board::new();
        assert_eq!(board.to_string(), "[]");
    }

    #[test]
    fn test_clear() {
        let mut board = make_flop();
        board.clear();
        assert!(board.is_empty());
        assert_eq!(board.street(), Street::Preflop);
    }
}
