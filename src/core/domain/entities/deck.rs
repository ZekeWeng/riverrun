//! Deck representation for poker.

use super::card::{Card, Rank, Suit};

/// A deck of cards.
#[derive(Clone, Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

/// Constructors
impl Deck {
    /// Create a new standard 52-card deck in order.
    pub fn new() -> Self {
        let cards = Rank::all()
            .flat_map(|rank| Suit::all().map(move |suit| Card::new(rank, suit)))
            .collect();
        Deck { cards }
    }

    /// Create an empty deck.
    pub fn empty() -> Self {
        Deck { cards: Vec::new() }
    }

    /// Create a deck from a vector of cards.
    pub fn from_cards(cards: Vec<Card>) -> Self {
        Deck { cards }
    }
}

/// Accessors
impl Deck {
    /// Get the number of remaining cards.
    pub fn remaining(&self) -> usize {
        self.cards.len()
    }

    /// Check if the deck is empty.
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Get all cards in the deck.
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    /// Peek at the top card without removing it.
    pub fn peek(&self) -> Option<&Card> {
        self.cards.last()
    }
}

/// Operations
impl Deck {
    /// Shuffle the deck using the provided random number generator.
    pub fn shuffle<R: rand::Rng>(&mut self, rng: &mut R) {
        use rand::seq::SliceRandom;
        self.cards.shuffle(rng);
    }

    /// Reset the deck to a full 52-card deck in order.
    pub fn reset(&mut self) {
        *self = Deck::new();
    }

    /// Remove specific cards from the deck (for dealing known cards).
    pub fn remove(&mut self, cards_to_remove: &[Card]) {
        self.cards.retain(|c| !cards_to_remove.contains(c));
    }

    /// Deal a single card from the top of the deck.
    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Deal exactly n cards, returning None if not enough cards.
    pub fn deal_exact(&mut self, n: usize) -> Option<Vec<Card>> {
        if self.cards.len() < n {
            return None;
        }
        Some(self.deal_n(n))
    }
}

/// Poker dealing
impl Deck {
    /// Deal hole cards to n players (2 cards each).
    /// Returns None if not enough cards for all players.
    pub fn deal_hole_cards(&mut self, num_players: usize) -> Option<Vec<[Card; 2]>> {
        let total_needed = num_players * 2;
        if self.cards.len() < total_needed {
            return None;
        }

        let mut hands = Vec::with_capacity(num_players);
        let mut first_cards = Vec::with_capacity(num_players);
        let mut second_cards = Vec::with_capacity(num_players);

        for _ in 0..num_players {
            first_cards.push(self.deal().unwrap());
        }
        for _ in 0..num_players {
            second_cards.push(self.deal().unwrap());
        }

        for i in 0..num_players {
            hands.push([first_cards[i], second_cards[i]]);
        }
        Some(hands)
    }

    /// Deal the flop (burn 1, deal 3).
    pub fn deal_flop(&mut self) -> Option<[Card; 3]> {
        let cards = self.burn_and_deal(3)?;
        Some([cards[0], cards[1], cards[2]])
    }

    /// Deal the turn (burn 1, deal 1).
    pub fn deal_turn(&mut self) -> Option<Card> {
        let cards = self.burn_and_deal(1)?;
        Some(cards[0])
    }

    /// Deal the river (burn 1, deal 1).
    pub fn deal_river(&mut self) -> Option<Card> {
        let cards = self.burn_and_deal(1)?;
        Some(cards[0])
    }
}

/// Private Helpers
impl Deck {
    fn deal_n(&mut self, n: usize) -> Vec<Card> {
        let start = self.cards.len().saturating_sub(n);
        self.cards.drain(start..).rev().collect()
    }

    fn burn(&mut self) -> Option<Card> {
        self.deal()
    }

    fn burn_and_deal(&mut self, n: usize) -> Option<Vec<Card>> {
        if self.cards.len() < n + 1 {
            return None;
        }
        self.burn();
        Some(self.deal_n(n))
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_deck_has_52_cards() {
        let deck = Deck::new();
        assert_eq!(deck.remaining(), 52);
    }

    #[test]
    fn test_empty_deck() {
        let deck = Deck::empty();
        assert!(deck.is_empty());
        assert_eq!(deck.remaining(), 0);
    }

    #[test]
    fn test_deal_reduces_count() {
        let mut deck = Deck::new();
        let card = deck.deal();
        assert!(card.is_some());
        assert_eq!(deck.remaining(), 51);
    }

    #[test]
    fn test_deal_from_empty() {
        let mut deck = Deck::empty();
        assert!(deck.deal().is_none());
    }

    #[test]
    fn test_deal_multiple() {
        let mut deck = Deck::new();
        let cards = deck.deal_exact(5).unwrap();
        assert_eq!(cards.len(), 5);
        assert_eq!(deck.remaining(), 47);
    }

    #[test]
    fn test_peek_does_not_remove() {
        let deck = Deck::new();
        let peeked = deck.peek();
        assert!(peeked.is_some());
        assert_eq!(deck.remaining(), 52);
    }

    #[test]
    fn test_remove_cards() {
        let mut deck = Deck::new();
        let cards_to_remove = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Hearts),
        ];
        deck.remove(&cards_to_remove);
        assert_eq!(deck.remaining(), 50);
    }

    #[test]
    fn test_reset() {
        let mut deck = Deck::new();
        deck.deal_n(10);
        assert_eq!(deck.remaining(), 42);
        deck.reset();
        assert_eq!(deck.remaining(), 52);
    }

    #[test]
    fn test_all_cards_unique() {
        let deck = Deck::new();
        let cards = deck.cards();
        let mut seen = std::collections::HashSet::new();
        for card in cards {
            assert!(seen.insert(card), "Duplicate card found: {}", card);
        }
    }

    #[test]
    fn test_from_cards() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Hearts),
        ];
        let deck = Deck::from_cards(cards);
        assert_eq!(deck.remaining(), 2);
    }

    #[test]
    fn test_deal_exact() {
        let mut deck = Deck::new();
        let cards = deck.deal_exact(5);
        assert!(cards.is_some());
        assert_eq!(cards.unwrap().len(), 5);
        assert_eq!(deck.remaining(), 47);
    }

    #[test]
    fn test_deal_exact_not_enough() {
        let mut deck = Deck::from_cards(vec![Card::new(Rank::Ace, Suit::Spades)]);
        assert!(deck.deal_exact(5).is_none());
        assert_eq!(deck.remaining(), 1); // Unchanged
    }

    #[test]
    fn test_deal_hole_cards() {
        let mut deck = Deck::new();
        let hands = deck.deal_hole_cards(4).unwrap();
        assert_eq!(hands.len(), 4);
        assert_eq!(deck.remaining(), 44); // 52 - 8
        for hand in &hands {
            assert_eq!(hand.len(), 2);
        }
    }

    #[test]
    fn test_deal_hole_cards_not_enough() {
        let mut deck = Deck::from_cards(vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Hearts),
        ]);
        assert!(deck.deal_hole_cards(2).is_none()); // Need 4 cards for 2 players
    }

    #[test]
    fn test_deal_flop() {
        let mut deck = Deck::new();
        let flop = deck.deal_flop().unwrap();
        assert_eq!(flop.len(), 3);
        assert_eq!(deck.remaining(), 48); // 52 - 1 burn - 3 dealt
    }

    #[test]
    fn test_deal_flop_not_enough_cards() {
        let mut deck = Deck::from_cards(vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Diamonds),
        ]);
        assert!(deck.deal_flop().is_none()); // Need 4 cards (1 burn + 3)
    }

    #[test]
    fn test_deal_turn() {
        let mut deck = Deck::new();
        let turn = deck.deal_turn().unwrap();
        assert_eq!(deck.remaining(), 50); // 52 - 1 burn - 1 turn
        assert!(turn.rank() <= 12);
    }

    #[test]
    fn test_deal_river() {
        let mut deck = Deck::new();
        let river = deck.deal_river().unwrap();
        assert_eq!(deck.remaining(), 50); // 52 - 1 burn - 1 river
        assert!(river.suit() <= 3);
    }

    #[test]
    fn test_full_deal_sequence() {
        let mut deck = Deck::new();
        let hands = deck.deal_hole_cards(6).unwrap(); // 12 cards
        assert_eq!(hands.len(), 6);
        assert_eq!(deck.remaining(), 40);

        let flop = deck.deal_flop().unwrap(); // burn + 3
        assert_eq!(flop.len(), 3);
        assert_eq!(deck.remaining(), 36);

        let turn = deck.deal_turn().unwrap(); // burn + 1
        assert!(turn.rank() <= 12);
        assert_eq!(deck.remaining(), 34);

        let river = deck.deal_river().unwrap(); // burn + 1
        assert!(river.suit() <= 3);
        assert_eq!(deck.remaining(), 32);
    }
}
