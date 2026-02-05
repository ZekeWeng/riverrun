//! Equity calculator using Monte Carlo simulation.
//!
//! Randomly samples opponent hands and board runouts to estimate equity.
//! Fast and suitable for all board states, especially preflop where
//! exhaustive enumeration is infeasible.

use crate::core::domain::entities::board::Board;
use crate::core::domain::entities::card::Card;
use crate::core::domain::entities::deck::Deck;
use crate::core::domain::entities::hole_cards::HoleCards;
use crate::core::ports::inbound::{EquityCalculator, EquityResult, HandEvaluator};

/// Default number of Monte Carlo iterations.
pub const DEFAULT_SAMPLES: u32 = 10_000;

pub struct MonteCarloEquityCalculator<E: HandEvaluator> {
    evaluator: E,
    default_samples: u32,
}

/// `MonteCarloEquityCalculator` - Constructors
impl<E: HandEvaluator> MonteCarloEquityCalculator<E> {
    /// Creates a MonteCarloEquityCalculator using the given evaluator and the module's default sample count.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::MonteCarloEquityCalculator;
    /// # use crate::CactusKevEvaluator;
    /// let eval = CactusKevEvaluator::new();
    /// let calc = MonteCarloEquityCalculator::new(eval);
    /// assert!(calc.default_samples() > 0);
    /// ```
    pub const fn new(evaluator: E) -> Self {
        Self {
            evaluator,
            default_samples: DEFAULT_SAMPLES,
        }
    }

    /// Creates a MonteCarloEquityCalculator with a custom default number of Monte Carlo samples.
    ///
    /// The `default_samples` value is used by calculation methods when no explicit sample count is provided.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming `evaluator` implements `HandEvaluator`
    /// let calc = MonteCarloEquityCalculator::with_samples(evaluator, 5_000);
    /// assert_eq!(calc.default_samples(), 5_000);
    /// ```
    pub const fn with_samples(evaluator: E, default_samples: u32) -> Self {
        Self {
            evaluator,
            default_samples,
        }
    }
}

/// `MonteCarloEquityCalculator` - Accessors
impl<E: HandEvaluator> MonteCarloEquityCalculator<E> {
    /// Get a reference to the underlying evaluator.
    pub const fn evaluator(&self) -> &E {
        &self.evaluator
    }

    /// Default number of Monte Carlo iterations used when a sample count is not provided.
    ///
    /// # Returns
    ///
    /// The default number of samples.
    ///
    /// # Examples
    ///
    /// ```
    /// // assuming `evaluator` implements `HandEvaluator`
    /// let calc = MonteCarloEquityCalculator::with_samples(evaluator, 1000);
    /// assert_eq!(calc.default_samples(), 1000);
    /// ```
    pub const fn default_samples(&self) -> u32 {
        self.default_samples
    }
}

/// `MonteCarloEquityCalculator` - Operations
impl<E: HandEvaluator> MonteCarloEquityCalculator<E> {
    /// Builds a deck containing all cards that are not present in the given hole cards and board.
    ///
    /// # Examples
    ///
    /// ```
    /// let hole = HoleCards::new(Card::AceSpades, Card::AceHearts);
    /// let board = Board::empty();
    /// let deck = remaining_deck(hole, &board);
    /// // pocket aces removed from the deck
    /// assert!(!deck.contains(&Card::AceSpades));
    /// assert!(!deck.contains(&Card::AceHearts));
    /// // full deck minus two hole cards
    /// assert_eq!(deck.len(), 50);
    /// ```
    fn remaining_deck(hole_cards: HoleCards, board: &Board) -> Deck {
        let mut dead_cards = vec![hole_cards.first(), hole_cards.second()];
        dead_cards.extend_from_slice(board.cards());
        Deck::excluding(&dead_cards)
    }
}

impl<E: HandEvaluator> EquityCalculator for MonteCarloEquityCalculator<E> {
    fn calculate(
        &self,
        hole_cards: &HoleCards,
        board: &Board,
        num_opponents: usize,
    ) -> EquityResult {
        self.calculate_sampled(hole_cards, board, num_opponents, self.default_samples)
    }

    /// Estimates the equity of the given hole cards against `num_opponents` by running a Monte Carlo
    /// simulation using `samples` random runouts and opponent hands.
    ///
    /// The function completes the board to five cards, samples opponent hole cards and remaining
    /// runout cards from the unseen deck, and returns aggregate win/tie/loss counts for the provided
    /// sample count.
    ///
    /// # Examples
    ///
    /// ```
    /// // Example (illustrative):
    /// // let evaluator = CactusKevEvaluator::new();
    /// // let calc = MonteCarloEquityCalculator::with_samples(evaluator, 1000);
    /// // let hole = HoleCards::new(Card::AceSpades, Card::AceHearts);
    /// // let board = Board::empty();
    /// // let result = calc.calculate_sampled(&hole, &board, 1, 500);
    /// // assert_eq!(result.samples(), 500);
    /// ```
    pub(crate)
    fn calculate_sampled(
        &self,
        hole_cards: &HoleCards,
        board: &Board,
        num_opponents: usize,
        samples: u32,
    ) -> EquityResult {
        let remaining = Self::remaining_deck(*hole_cards, board);
        let cards_to_deal = 5 - board.len();

        self.simulate(*hole_cards, board.cards(), &remaining, num_opponents, cards_to_deal, samples)
    }
}

/// `MonteCarloEquityCalculator` - Simulation
impl<E: HandEvaluator> MonteCarloEquityCalculator<E> {
    /// Performs a Monte Carlo simulation to estimate equity for the given hole cards and board.
    ///
    /// The simulation repeatedly samples remaining unseen cards to complete the board and deal opponent
    /// hole cards, evaluates each player's 7-card hand with the configured evaluator, and accumulates
    /// win/tie/loss counts across `iterations`.
    ///
    /// # Parameters
    ///
    /// - `hole_cards`: the hero's two hole cards.
    /// - `board_cards`: the current shared board cards (0..5 cards).
    /// - `remaining`: deck of unseen cards to sample from (must exclude `hole_cards` and `board_cards`).
    /// - `num_opponents`: number of opponents to simulate (each receives two hole cards).
    /// - `cards_to_deal`: number of runout cards to deal to complete a 5-card board (0..5 - `board_cards.len()`).
    /// - `iterations`: number of Monte Carlo samples to perform.
    ///
    /// # Returns
    ///
    /// An `EquityResult` constructed from the accumulated win, tie, and loss counts for the hero against
    /// `num_opponents`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let calc = MonteCarloEquityCalculator::new(evaluator);
    /// let result = calc.simulate(hole_cards, &board_cards, &remaining_deck, 2, 3, 10_000);
    /// println!("wins: {}, ties: {}, losses: {}", result.wins(), result.ties(), result.losses());
    /// ```
    fn simulate(
        &self,
        hole_cards: HoleCards,
        board_cards: &[Card],
        remaining: &Deck,
        num_opponents: usize,
        cards_to_deal: usize,
        iterations: u32,
    ) -> EquityResult {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let cards = remaining.to_vec();
        let mut wins = 0u64;
        let mut ties = 0u64;
        let mut losses = 0u64;

        // Deterministic seed for reproducibility
        let mut hasher = DefaultHasher::new();
        hole_cards.first().index().hash(&mut hasher);
        hole_cards.second().index().hash(&mut hasher);
        board_cards.len().hash(&mut hasher);
        let mut seed = hasher.finish();

        let total_cards_needed = cards_to_deal + (num_opponents * 2);

        if cards.len() < total_cards_needed {
            return EquityResult::from_counts(0, 0, 0, num_opponents);
        }

        for _ in 0..iterations {
            // Fisher-Yates partial shuffle using LCG
            let mut shuffled = cards.clone();
            for i in 0..total_cards_needed {
                seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
                let j = i + ((seed >> 33) as usize % (shuffled.len() - i));
                shuffled.swap(i, j);
            }

            // Build complete board
            let mut full_board = [Card::from_index(0).unwrap(); 5];
            for (i, &card) in board_cards.iter().enumerate() {
                full_board[i] = card;
            }
            for i in 0..cards_to_deal {
                full_board[board_cards.len() + i] = shuffled[i];
            }

            let hero_cards = hole_cards.combine_with_board(full_board);
            let hero_strength = self.evaluator.evaluate_7cards_fast(&hero_cards);

            // Evaluate opponents
            let mut hero_wins = true;
            let mut any_tie = false;
            let opp_start = cards_to_deal;

            for opp in 0..num_opponents {
                let opp_idx = opp_start + (opp * 2);
                let opp_hole = HoleCards::new(shuffled[opp_idx], shuffled[opp_idx + 1]);
                let opp_cards = opp_hole.combine_with_board(full_board);
                let opp_strength = self.evaluator.evaluate_7cards_fast(&opp_cards);

                if opp_strength < hero_strength {
                    hero_wins = false;
                    break;
                } else if opp_strength == hero_strength {
                    any_tie = true;
                }
            }

            if !hero_wins {
                losses += 1;
            } else if any_tie {
                ties += 1;
            } else {
                wins += 1;
            }
        }

        EquityResult::from_counts(wins, ties, losses, num_opponents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::entities::card::{Rank, Suit};
    use crate::core::domain::services::evaluation::CactusKevEvaluator;

    fn card(rank: Rank, suit: Suit) -> Card {
        Card::new(rank, suit)
    }

    fn make_board(cards: Vec<Card>) -> Board {
        Board::with_cards(cards).unwrap()
    }

    #[test]
    fn test_preflop_pocket_aces() {
        let calc = MonteCarloEquityCalculator::with_samples(CactusKevEvaluator::new(), 5000);

        let hole_cards = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
        );

        let board = Board::new(); // Preflop

        let result = calc.calculate(&hole_cards, &board, 1);

        // Pocket aces preflop ~85% equity heads-up
        assert!(result.equity() > 0.80);
        assert!(result.equity() < 0.90);
    }

    #[test]
    fn test_preflop_72_offsuit() {
        let calc = MonteCarloEquityCalculator::with_samples(CactusKevEvaluator::new(), 5000);

        let hole_cards = HoleCards::new(
            card(Rank::Seven, Suit::Spades),
            card(Rank::Two, Suit::Hearts),
        );

        let board = Board::new();

        let result = calc.calculate(&hole_cards, &board, 1);

        // 72o is worst hand, ~35% equity heads-up
        assert!(result.equity() > 0.30);
        assert!(result.equity() < 0.40);
    }

    #[test]
    fn test_flop_flush_draw() {
        let calc = MonteCarloEquityCalculator::with_samples(CactusKevEvaluator::new(), 5000);

        let hole_cards = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
        );

        let board = make_board(vec![
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Two, Suit::Hearts),
        ]);

        let result = calc.calculate(&hole_cards, &board, 1);

        // AKs with nut flush draw + straight draw
        assert!(result.equity() > 0.50);
    }

    #[test]
    fn test_multi_opponent() {
        let calc = MonteCarloEquityCalculator::with_samples(CactusKevEvaluator::new(), 5000);

        let hole_cards = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
        );

        let board = Board::new();

        let result_1 = calc.calculate(&hole_cards, &board, 1);
        let result_3 = calc.calculate(&hole_cards, &board, 3);
        let result_5 = calc.calculate(&hole_cards, &board, 5);

        // Equity decreases with more opponents
        assert!(result_1.equity() > result_3.equity());
        assert!(result_3.equity() > result_5.equity());
    }

    #[test]
    fn test_custom_samples() {
        let calc = MonteCarloEquityCalculator::with_samples(CactusKevEvaluator::new(), 1000);

        assert_eq!(calc.default_samples(), 1000);

        let hole_cards = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
        );

        let board = Board::new();

        // Using custom sample count
        let result = calc.calculate_sampled(&hole_cards, &board, 1, 500);
        assert_eq!(result.samples(), 500);
    }

    #[test]
    fn test_river_equity() {
        let calc = MonteCarloEquityCalculator::new(CactusKevEvaluator::new());

        let hole_cards = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
        );

        let board = make_board(vec![
            card(Rank::King, Suit::Diamonds),
            card(Rank::Queen, Suit::Clubs),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Two, Suit::Hearts),
            card(Rank::Seven, Suit::Clubs),
        ]);

        let result = calc.calculate(&hole_cards, &board, 1);

        // Pocket aces on safe board
        assert!(result.equity() > 0.80);
    }
}