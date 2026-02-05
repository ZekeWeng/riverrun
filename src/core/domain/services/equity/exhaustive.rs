//! Equity calculator using exhaustive enumeration.
//!
//! Enumerates all possible opponent hands and board runouts to calculate
//! exact equity.
//!
//! For preflop, consider using `MonteCarloEquityCalculator` instead.

use crate::core::domain::entities::board::Board;
use crate::core::domain::entities::card::Card;
use crate::core::domain::entities::deck::Deck;
use crate::core::domain::entities::hole_cards::HoleCards;
use crate::core::ports::inbound::{EquityCalculator, EquityResult, HandEvaluator};

pub struct ExhaustiveEquityCalculator<E: HandEvaluator> {
    evaluator: E,
}

/// `ExhaustiveEquityCalculator` - Constructors
impl<E: HandEvaluator> ExhaustiveEquityCalculator<E> {
    /// Create a new exhaustive equity calculator.
    pub const fn new(evaluator: E) -> Self {
        Self { evaluator }
    }
}

/// `ExhaustiveEquityCalculator` - Accessors
impl<E: HandEvaluator> ExhaustiveEquityCalculator<E> {
    /// Get a reference to the underlying evaluator.
    pub const fn evaluator(&self) -> &E {
        &self.evaluator
    }
}

/// `ExhaustiveEquityCalculator` - Operations
impl<E: HandEvaluator> ExhaustiveEquityCalculator<E> {
    /// Create a deck with dead cards removed.
    fn remaining_deck(hole_cards: HoleCards, board: &Board) -> Deck {
        let mut dead_cards = vec![hole_cards.first(), hole_cards.second()];
        dead_cards.extend_from_slice(board.cards());
        Deck::excluding(&dead_cards)
    }
}

impl<E: HandEvaluator> EquityCalculator for ExhaustiveEquityCalculator<E> {
    fn calculate(
        &self,
        hole_cards: &HoleCards,
        board: &Board,
        num_opponents: usize,
    ) -> EquityResult {
        let remaining = Self::remaining_deck(*hole_cards, board);

        match board.len() {
            5 => self.calculate_river(*hole_cards, board, &remaining, num_opponents),
            4 => self.calculate_turn(*hole_cards, board, &remaining, num_opponents),
            3 => self.calculate_flop(*hole_cards, board, &remaining, num_opponents),
            0 => self.calculate_preflop(*hole_cards, &remaining, num_opponents),
            _ => EquityResult::from_counts(0, 0, 0, num_opponents),
        }
    }

    fn calculate_sampled(
        &self,
        hole_cards: &HoleCards,
        board: &Board,
        num_opponents: usize,
        _samples: u32,
    ) -> EquityResult {
        // Exhaustive calculator ignores sample count - always does full enumeration
        self.calculate(hole_cards, board, num_opponents)
    }
}

/// `ExhaustiveEquityCalculator` - Calculation Methods
impl<E: HandEvaluator> ExhaustiveEquityCalculator<E> {
    /// Calculate equity on the river using exhaustive enumeration.
    fn calculate_river(
        &self,
        hole_cards: HoleCards,
        board: &Board,
        remaining: &Deck,
        num_opponents: usize,
    ) -> EquityResult {
        let board_array = board.as_array().unwrap();
        let hero_cards = hole_cards.combine_with_board(board_array);
        let hero_strength = self.evaluator.evaluate_7cards_fast(&hero_cards);

        let mut wins = 0u64;
        let mut ties = 0u64;
        let mut losses = 0u64;

        let cards = remaining.cards();

        if num_opponents == 1 {
            for i in 0..cards.len() {
                for j in (i + 1)..cards.len() {
                    let opp_hole = HoleCards::new(cards[i], cards[j]);
                    let opp_cards = opp_hole.combine_with_board(board_array);
                    let opp_strength = self.evaluator.evaluate_7cards_fast(&opp_cards);

                    match hero_strength.cmp(&opp_strength) {
                        std::cmp::Ordering::Less => wins += 1,
                        std::cmp::Ordering::Equal => ties += 1,
                        std::cmp::Ordering::Greater => losses += 1,
                    }
                }
            }
        } else {
            // Multi-way exhaustive is expensive but possible for small opponent counts
            self.enumerate_multiway(hole_cards, &board_array, remaining, num_opponents, &mut wins, &mut ties, &mut losses);
        }

        EquityResult::from_counts(wins, ties, losses, num_opponents)
    }

    /// Calculate equity on the turn using exhaustive enumeration.
    fn calculate_turn(
        &self,
        hole_cards: HoleCards,
        board: &Board,
        remaining: &Deck,
        num_opponents: usize,
    ) -> EquityResult {
        let board_cards = board.cards();
        let cards = remaining.cards();
        let mut wins = 0u64;
        let mut ties = 0u64;
        let mut losses = 0u64;

        if num_opponents == 1 {
            for (river_idx, &river_card) in cards.iter().enumerate() {
                let full_board = [
                    board_cards[0],
                    board_cards[1],
                    board_cards[2],
                    board_cards[3],
                    river_card,
                ];

                let hero_cards = hole_cards.combine_with_board(full_board);
                let hero_strength = self.evaluator.evaluate_7cards_fast(&hero_cards);

                for i in 0..cards.len() {
                    if i == river_idx {
                        continue;
                    }
                    for j in (i + 1)..cards.len() {
                        if j == river_idx {
                            continue;
                        }

                        let opp_hole = HoleCards::new(cards[i], cards[j]);
                        let opp_cards = opp_hole.combine_with_board(full_board);
                        let opp_strength = self.evaluator.evaluate_7cards_fast(&opp_cards);

                        match hero_strength.cmp(&opp_strength) {
                            std::cmp::Ordering::Less => wins += 1,
                            std::cmp::Ordering::Equal => ties += 1,
                            std::cmp::Ordering::Greater => losses += 1,
                        }
                    }
                }
            }
        } else {
            // For multi-way on turn, enumerate each river then multiway
            for (river_idx, &river_card) in cards.iter().enumerate() {
                let full_board = [
                    board_cards[0],
                    board_cards[1],
                    board_cards[2],
                    board_cards[3],
                    river_card,
                ];

                let remaining_after_river: Vec<_> = cards.iter()
                    .enumerate()
                    .filter(|(i, _)| *i != river_idx)
                    .map(|(_, &c)| c)
                    .collect();
                let river_deck = Deck::from_cards(remaining_after_river);

                self.enumerate_multiway(hole_cards, &full_board, &river_deck, num_opponents, &mut wins, &mut ties, &mut losses);
            }
        }

        EquityResult::from_counts(wins, ties, losses, num_opponents)
    }

    /// Calculate equity on the flop using exhaustive enumeration.
    fn calculate_flop(
        &self,
        hole_cards: HoleCards,
        board: &Board,
        remaining: &Deck,
        num_opponents: usize,
    ) -> EquityResult {
        let board_cards = board.cards();
        let cards = remaining.cards();
        let mut wins = 0u64;
        let mut ties = 0u64;
        let mut losses = 0u64;

        if num_opponents == 1 {
            for turn_idx in 0..cards.len() {
                for river_idx in (turn_idx + 1)..cards.len() {
                    let full_board = [
                        board_cards[0],
                        board_cards[1],
                        board_cards[2],
                        cards[turn_idx],
                        cards[river_idx],
                    ];

                    let hero_cards = hole_cards.combine_with_board(full_board);
                    let hero_strength = self.evaluator.evaluate_7cards_fast(&hero_cards);

                    for i in 0..cards.len() {
                        if i == turn_idx || i == river_idx {
                            continue;
                        }
                        for j in (i + 1)..cards.len() {
                            if j == turn_idx || j == river_idx {
                                continue;
                            }

                            let opp_hole = HoleCards::new(cards[i], cards[j]);
                            let opp_cards = opp_hole.combine_with_board(full_board);
                            let opp_strength = self.evaluator.evaluate_7cards_fast(&opp_cards);

                            match hero_strength.cmp(&opp_strength) {
                                std::cmp::Ordering::Less => wins += 1,
                                std::cmp::Ordering::Equal => ties += 1,
                                std::cmp::Ordering::Greater => losses += 1,
                            }
                        }
                    }
                }
            }
        } else {
            // Multi-way flop enumeration - very expensive
            for turn_idx in 0..cards.len() {
                for river_idx in (turn_idx + 1)..cards.len() {
                    let full_board = [
                        board_cards[0],
                        board_cards[1],
                        board_cards[2],
                        cards[turn_idx],
                        cards[river_idx],
                    ];

                    let remaining_cards: Vec<_> = cards.iter()
                        .enumerate()
                        .filter(|(i, _)| *i != turn_idx && *i != river_idx)
                        .map(|(_, &c)| c)
                        .collect();
                    let runout_deck = Deck::from_cards(remaining_cards);

                    self.enumerate_multiway(hole_cards, &full_board, &runout_deck, num_opponents, &mut wins, &mut ties, &mut losses);
                }
            }
        }

        EquityResult::from_counts(wins, ties, losses, num_opponents)
    }

    /// Calculate preflop equity using exhaustive enumeration (very slow!).
    fn calculate_preflop(
        &self,
        hole_cards: HoleCards,
        remaining: &Deck,
        num_opponents: usize,
    ) -> EquityResult {
        let cards = remaining.cards();
        let mut wins = 0u64;
        let mut ties = 0u64;
        let mut losses = 0u64;

        if num_opponents == 1 {
            // Enumerate all boards and opponent hands
            for b0 in 0..cards.len() {
                for b1 in (b0 + 1)..cards.len() {
                    for b2 in (b1 + 1)..cards.len() {
                        for b3 in (b2 + 1)..cards.len() {
                            for b4 in (b3 + 1)..cards.len() {
                                let full_board = [cards[b0], cards[b1], cards[b2], cards[b3], cards[b4]];
                                let hero_cards = hole_cards.combine_with_board(full_board);
                                let hero_strength = self.evaluator.evaluate_7cards_fast(&hero_cards);

                                let board_indices = [b0, b1, b2, b3, b4];
                                for i in 0..cards.len() {
                                    if board_indices.contains(&i) {
                                        continue;
                                    }
                                    for j in (i + 1)..cards.len() {
                                        if board_indices.contains(&j) {
                                            continue;
                                        }

                                        let opp_hole = HoleCards::new(cards[i], cards[j]);
                                        let opp_cards = opp_hole.combine_with_board(full_board);
                                        let opp_strength = self.evaluator.evaluate_7cards_fast(&opp_cards);

                                        match hero_strength.cmp(&opp_strength) {
                                            std::cmp::Ordering::Less => wins += 1,
                                            std::cmp::Ordering::Equal => ties += 1,
                                            std::cmp::Ordering::Greater => losses += 1,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            // Multi-way preflop exhaustive is computationally infeasible
            // Return empty result - user should use Monte Carlo instead
            return EquityResult::from_counts(0, 0, 0, num_opponents);
        }

        EquityResult::from_counts(wins, ties, losses, num_opponents)
    }

    /// Enumerate all multiway opponent combinations for a complete board.
    #[allow(clippy::too_many_arguments)]
    fn enumerate_multiway(
        &self,
        hole_cards: HoleCards,
        board: &[Card; 5],
        remaining: &Deck,
        num_opponents: usize,
        wins: &mut u64,
        ties: &mut u64,
        losses: &mut u64,
    ) {
        if num_opponents > 3 {
            // Too many opponents for exhaustive enumeration
            return;
        }

        let cards = remaining.cards();
        let hero_cards = hole_cards.combine_with_board(*board);
        let hero_strength = self.evaluator.evaluate_7cards_fast(&hero_cards);

        match num_opponents {
            2 => {
                // 2 opponents: enumerate all ways to give them 2 cards each
                for o1_0 in 0..cards.len() {
                    for o1_1 in (o1_0 + 1)..cards.len() {
                        for o2_0 in 0..cards.len() {
                            if o2_0 == o1_0 || o2_0 == o1_1 {
                                continue;
                            }
                            for o2_1 in (o2_0 + 1)..cards.len() {
                                if o2_1 == o1_0 || o2_1 == o1_1 {
                                    continue;
                                }

                                let opp1 = HoleCards::new(cards[o1_0], cards[o1_1]);
                                let opp2 = HoleCards::new(cards[o2_0], cards[o2_1]);

                                let s1 = self.evaluator.evaluate_7cards_fast(&opp1.combine_with_board(*board));
                                let s2 = self.evaluator.evaluate_7cards_fast(&opp2.combine_with_board(*board));

                                let best_opp = s1.min(s2);

                                match hero_strength.cmp(&best_opp) {
                                    std::cmp::Ordering::Less => *wins += 1,
                                    std::cmp::Ordering::Equal => *ties += 1,
                                    std::cmp::Ordering::Greater => *losses += 1,
                                }
                            }
                        }
                    }
                }
            }
            3 => {
                // 3 opponents - even more expensive
                for o1_0 in 0..cards.len() {
                    for o1_1 in (o1_0 + 1)..cards.len() {
                        for o2_0 in 0..cards.len() {
                            if o2_0 == o1_0 || o2_0 == o1_1 {
                                continue;
                            }
                            for o2_1 in (o2_0 + 1)..cards.len() {
                                if o2_1 == o1_0 || o2_1 == o1_1 {
                                    continue;
                                }
                                for o3_0 in 0..cards.len() {
                                    if o3_0 == o1_0 || o3_0 == o1_1 || o3_0 == o2_0 || o3_0 == o2_1 {
                                        continue;
                                    }
                                    for o3_1 in (o3_0 + 1)..cards.len() {
                                        if o3_1 == o1_0 || o3_1 == o1_1 || o3_1 == o2_0 || o3_1 == o2_1 {
                                            continue;
                                        }

                                        let opp1 = HoleCards::new(cards[o1_0], cards[o1_1]);
                                        let opp2 = HoleCards::new(cards[o2_0], cards[o2_1]);
                                        let opp3 = HoleCards::new(cards[o3_0], cards[o3_1]);

                                        let s1 = self.evaluator.evaluate_7cards_fast(&opp1.combine_with_board(*board));
                                        let s2 = self.evaluator.evaluate_7cards_fast(&opp2.combine_with_board(*board));
                                        let s3 = self.evaluator.evaluate_7cards_fast(&opp3.combine_with_board(*board));

                                        let best_opp = s1.min(s2).min(s3);

                                        match hero_strength.cmp(&best_opp) {
                                            std::cmp::Ordering::Less => *wins += 1,
                                            std::cmp::Ordering::Equal => *ties += 1,
                                            std::cmp::Ordering::Greater => *losses += 1,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                // For 1 opponent, use the simpler loop in the caller
                // For >3 opponents, not supported
            }
        }
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
    fn test_river_equity_pocket_aces() {
        let calc = ExhaustiveEquityCalculator::new(CactusKevEvaluator::new());

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
        assert!(result.equity() > 0.85);
    }

    #[test]
    fn test_river_equity_weak_hand() {
        let calc = ExhaustiveEquityCalculator::new(CactusKevEvaluator::new());

        let hole_cards = HoleCards::new(
            card(Rank::Two, Suit::Spades),
            card(Rank::Three, Suit::Hearts),
        );

        let board = make_board(vec![
            card(Rank::King, Suit::Diamonds),
            card(Rank::Queen, Suit::Clubs),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Nine, Suit::Hearts),
            card(Rank::Eight, Suit::Clubs),
        ]);

        let result = calc.calculate(&hole_cards, &board, 1);
        assert!(result.equity() < 0.20);
    }
}
