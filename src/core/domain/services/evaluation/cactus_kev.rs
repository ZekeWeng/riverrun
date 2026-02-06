//! Hand evaluator using Cactus Kev's algorithm.
//!
//! Uses precomputed lookup tables for fast evaluation:
//! - Flush hands: O(1) lookup via rank bits
//! - Non-flush hands: O(log n) binary search via prime product

use crate::core::domain::entities::card::Card;
use crate::core::domain::entities::hand::Hand;
use crate::core::ports::inbound::HandEvaluator;

use super::super::utils::FIVE_FROM_SEVEN;
use super::hand_rank_tables::HandRankTables;

/// Hand evaluator using Cactus Kev's prime product algorithm.
pub struct CactusKevEvaluator {
    tables: HandRankTables,
}

/// `CactusKevEvaluator` - Constructors
impl CactusKevEvaluator {
    /// Constructs a `CactusKevEvaluator` initialized with the default precomputed hand-rank tables.
    #[must_use] 
    pub fn new() -> Self {
        Self {
            tables: HandRankTables::new(),
        }
    }

    /// Create an evaluator that uses the provided precomputed hand rank tables.
    ///
    /// The `tables` argument supplies the precomputed lookup data used for fast hand evaluation.
    #[must_use] 
    pub const fn with_tables(tables: HandRankTables) -> Self {
        Self { tables }
    }
}

/// `CactusKevEvaluator` - Accessors
impl CactusKevEvaluator {
    /// Provides access to the evaluator's precomputed hand-rank lookup tables.
    ///
    /// Returns a reference to the underlying `HandRankTables`. 
    #[must_use] 
    pub const fn tables(&self) -> &HandRankTables {
        &self.tables
    }
}

impl Default for CactusKevEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl HandEvaluator for CactusKevEvaluator {
    fn evaluate_5cards(&self, cards: [Card; 5]) -> Hand {
        let strength = self.evaluate_5cards_fast(&cards);
        Hand::new(cards, strength)
    }

    fn evaluate_7cards(&self, cards: [Card; 7]) -> Hand {
        let mut best_rank = u16::MAX;
        let mut best_cards = [cards[0], cards[1], cards[2], cards[3], cards[4]];

        for combo in FIVE_FROM_SEVEN {
            let hand_cards = [
                cards[combo[0]],
                cards[combo[1]],
                cards[combo[2]],
                cards[combo[3]],
                cards[combo[4]],
            ];

            let rank = self.evaluate_5cards_fast(&hand_cards);

            if rank == 1 {
                return Hand::new(hand_cards, 1);
            }

            if rank < best_rank {
                best_rank = rank;
                best_cards = hand_cards;
            }
        }

        Hand::new(best_cards, best_rank)
    }

    /// Evaluate a five-card hand and return its canonical strength code.
    ///
    /// Computes the hand strength for exactly five `Card`s: if all five share a suit it returns
    /// the flush-based lookup result; otherwise it looks up the hand by the product of card primes.
    ///
    /// # Returns
    ///
    /// The hand strength as a `u16` (lower is stronger).
    ///
    /// # Panics
    ///
    /// Panics if the prime-product lookup does not yield a known hand rank.
    #[inline]
    fn evaluate_5cards_fast(&self, cards: &[Card; 5]) -> u16 {
        let suit_and = cards[0].0 & cards[1].0 & cards[2].0 & cards[3].0 & cards[4].0 & 0xF000;

        if suit_and != 0 {
            let rank_bits = cards[0].rank_bits()
                | cards[1].rank_bits()
                | cards[2].rank_bits()
                | cards[3].rank_bits()
                | cards[4].rank_bits();
            return self.tables.lookup_flush(rank_bits);
        }

        let prime_product = cards[0].prime()
            * cards[1].prime()
            * cards[2].prime()
            * cards[3].prime()
            * cards[4].prime();

        self.tables
            .lookup_unique(prime_product)
            .unwrap_or_else(|| panic!("Invalid hand with prime product: {prime_product}"))
    }

    /// Evaluates seven cards by testing every five-card combination and returning the best hand rank.
    ///
    /// Tries each 5-card subset of the provided seven cards, computes its rank, and returns the lowest
    /// (best) rank found. Evaluation stops early if a rank of `1` (royal flush) is encountered.
    ///
    /// # Returns
    ///
    /// `u16` containing the best hand rank found; lower values represent stronger hands (1 is a royal flush).
    fn evaluate_7cards_fast(&self, cards: &[Card; 7]) -> u16 {
        let mut best_rank = u16::MAX;

        for combo in FIVE_FROM_SEVEN {
            let hand = [
                cards[combo[0]],
                cards[combo[1]],
                cards[combo[2]],
                cards[combo[3]],
                cards[combo[4]],
            ];

            let rank = self.evaluate_5cards_fast(&hand);

            if rank == 1 {
                return 1;
            }

            if rank < best_rank {
                best_rank = rank;
            }
        }

        best_rank
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
    fn test_royal_flush() {
        let evaluator = CactusKevEvaluator::new();
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Spades),
        ];
        let hand = evaluator.evaluate_5cards(cards);
        assert!(hand.is_royal_flush());
        assert_eq!(hand.strength(), 1);
    }

    #[test]
    fn test_straight_flush() {
        let evaluator = CactusKevEvaluator::new();
        let cards = [
            card(Rank::Nine, Suit::Hearts),
            card(Rank::Eight, Suit::Hearts),
            card(Rank::Seven, Suit::Hearts),
            card(Rank::Six, Suit::Hearts),
            card(Rank::Five, Suit::Hearts),
        ];
        let hand = evaluator.evaluate_5cards(cards);
        assert!(hand.is_straight_flush());
        assert!(!hand.is_royal_flush());
    }

    #[test]
    fn test_four_of_a_kind() {
        let evaluator = CactusKevEvaluator::new();
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::Ace, Suit::Clubs),
            card(Rank::King, Suit::Spades),
        ];
        let hand = evaluator.evaluate_5cards(cards);
        assert!(hand.is_four_of_a_kind());
    }

    #[test]
    fn test_full_house() {
        let evaluator = CactusKevEvaluator::new();
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
            card(Rank::King, Suit::Spades),
        ];
        let hand = evaluator.evaluate_5cards(cards);
        assert!(hand.is_full_house());
    }

    #[test]
    fn test_flush() {
        let evaluator = CactusKevEvaluator::new();
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Eight, Suit::Spades),
            card(Rank::Four, Suit::Spades),
            card(Rank::Two, Suit::Spades),
        ];
        let hand = evaluator.evaluate_5cards(cards);
        assert!(hand.is_flush());
    }

    #[test]
    fn test_straight() {
        let evaluator = CactusKevEvaluator::new();
        let cards = [
            card(Rank::Nine, Suit::Spades),
            card(Rank::Eight, Suit::Hearts),
            card(Rank::Seven, Suit::Diamonds),
            card(Rank::Six, Suit::Clubs),
            card(Rank::Five, Suit::Spades),
        ];
        let hand = evaluator.evaluate_5cards(cards);
        assert!(hand.is_straight());
    }

    #[test]
    fn test_wheel_straight() {
        let evaluator = CactusKevEvaluator::new();
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::Two, Suit::Hearts),
            card(Rank::Three, Suit::Diamonds),
            card(Rank::Four, Suit::Clubs),
            card(Rank::Five, Suit::Spades),
        ];
        let hand = evaluator.evaluate_5cards(cards);
        assert!(hand.is_straight());
    }

    #[test]
    fn test_three_of_a_kind() {
        let evaluator = CactusKevEvaluator::new();
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
            card(Rank::Queen, Suit::Spades),
        ];
        let hand = evaluator.evaluate_5cards(cards);
        assert!(hand.is_three_of_a_kind());
    }

    #[test]
    fn test_two_pair() {
        let evaluator = CactusKevEvaluator::new();
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
            card(Rank::Queen, Suit::Spades),
        ];
        let hand = evaluator.evaluate_5cards(cards);
        assert!(hand.is_two_pair());
    }

    #[test]
    fn test_one_pair() {
        let evaluator = CactusKevEvaluator::new();
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Diamonds),
            card(Rank::Queen, Suit::Clubs),
            card(Rank::Jack, Suit::Spades),
        ];
        let hand = evaluator.evaluate_5cards(cards);
        assert!(hand.is_one_pair());
    }

    #[test]
    fn test_high_card() {
        let evaluator = CactusKevEvaluator::new();
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Diamonds),
            card(Rank::Jack, Suit::Clubs),
            card(Rank::Nine, Suit::Spades),
        ];
        let hand = evaluator.evaluate_5cards(cards);
        assert!(hand.is_high_card());
    }

    #[test]
    fn test_7card_evaluation() {
        let evaluator = CactusKevEvaluator::new();
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Spades),
            card(Rank::Two, Suit::Hearts),
            card(Rank::Three, Suit::Diamonds),
        ];
        let hand = evaluator.evaluate_7cards(cards);
        assert!(hand.is_royal_flush());
    }

    #[test]
    fn test_hand_ordering() {
        let evaluator = CactusKevEvaluator::new();

        let royal_flush = evaluator.evaluate_5cards([
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Spades),
        ]);

        let four_kind = evaluator.evaluate_5cards([
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::Ace, Suit::Clubs),
            card(Rank::King, Suit::Spades),
        ]);

        let high_card = evaluator.evaluate_5cards([
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Diamonds),
            card(Rank::Jack, Suit::Clubs),
            card(Rank::Nine, Suit::Spades),
        ]);

        assert!(royal_flush.beats(&four_kind));
        assert!(four_kind.beats(&high_card));
        assert!(royal_flush.beats(&high_card));
    }
}