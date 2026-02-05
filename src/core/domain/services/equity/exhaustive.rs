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
    /// Access the underlying hand evaluator.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Obtain a reference to the evaluator backing the calculator.
    /// let evaluator_ref = calc.evaluator();
    /// ```
    pub const fn evaluator(&self) -> &E {
        &self.evaluator
    }
}

/// `ExhaustiveEquityCalculator` - Operations
impl<E: HandEvaluator> ExhaustiveEquityCalculator<E> {
    /// Builds a deck excluding the given hole cards and board cards.
    ///
    /// The returned `Deck` contains all cards except the two `hole_cards` and any cards present on `board`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let hole = HoleCards::new(Card::from_str("As").unwrap(), Card::from_str("Ah").unwrap());
    /// let board = Board::from_cards(&[]);
    /// let deck = remaining_deck(hole, &board);
    /// // 52 total cards minus 2 hole cards
    /// assert_eq!(deck.count(), 50);
    /// ```
    fn remaining_deck(hole_cards: HoleCards, board: &Board) -> Deck {
        let mut dead_cards = vec![hole_cards.first(), hole_cards.second()];
        dead_cards.extend_from_slice(board.cards());
        Deck::excluding(&dead_cards)
    }
}

impl<E: HandEvaluator> EquityCalculator for ExhaustiveEquityCalculator<E> {
    /// Selects and runs the appropriate exhaustive equity calculation for the given board stage.
    ///
    /// The function builds the remaining deck from the hero's hole cards and the board, then dispatches
    /// to the river/turn/flop/preflop calculation implementation depending on how many board cards
    /// are present. If the board length is an unsupported intermediate size, returns zeroed counts.
    ///
    /// # Returns
    ///
    /// An `EquityResult` containing aggregated win/tie/loss counts for the provided `num_opponents`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // `calc` is an ExhaustiveEquityCalculator initialized with a hand evaluator.
    /// // `hole` is the hero's HoleCards and `board` is a Board (0..5 cards).
    /// let result = calc.calculate(&hole, &board, 1);
    /// ```
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

    /// Calculates exact equity using full enumeration, ignoring any requested sample count.
    ///
    /// The `_samples` argument is ignored; this method performs an exhaustive calculation and returns the exact
    /// equity for the provided hole cards, board, and number of opponents.
    ///
    /// Returns the computed `EquityResult` for the given inputs.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let eval = CactusKevEvaluator::new();
    /// let calc = ExhaustiveEquityCalculator::new(eval);
    /// let hole = HoleCards::new(Card::ace_spades(), Card::ace_hearts());
    /// let board = Board::from_cards(&[]);
    /// let result = calc.calculate_sampled(&hole, &board, 1, 100);
    /// assert!(result.wins + result.ties + result.losses > 0);
    /// ```
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

    /// Computes exact equity on the turn by enumerating all possible river cards and opponent hole cards.
    ///
    /// Returns an `EquityResult` containing counts of wins, ties, and losses for the given number of opponents
    /// after considering every legal river runout. For single-opponent scenarios this enumerates opponent hole
    /// combinations directly; for multiway scenarios it enumerates each river and delegates to the multiway
    /// enumerator.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let evaluator = CactusKevEvaluator::new();
    /// let calc = ExhaustiveEquityCalculator::new(evaluator);
    /// let hole = HoleCards::new(Card::from_str("As").unwrap(), Card::from_str("Ah").unwrap());
    /// let board = Board::from_cards([
    ///     Card::from_str("Kd").unwrap(),
    ///     Card::from_str("Qc").unwrap(),
    ///     Card::from_str("2s").unwrap(),
    ///     Card::from_str("7h").unwrap(),
    /// ]);
    /// let remaining = Deck::from_full_deck_excluding(&hole, &board);
    ///
    /// let result = calc.calculate_turn(hole, &board, &remaining, 1);
    /// assert!(result.wins + result.ties + result.losses > 0);
    /// ```
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

    /// Computes exact equity from the flop by exhaustively enumerating all possible turn and river cards
    /// and all valid opponent hole-card combinations.
    ///
    /// For `num_opponents == 1` this evaluates every turn+river runout and every opponent two-card hand
    /// to tally wins, ties, and losses for the hero. For `num_opponents > 1` this delegates to the
    /// multiway enumerator (which is significantly more expensive and may early-return for unsupported
    /// opponent counts, yielding zeroed results).
    ///
    /// # Parameters
    ///
    /// - `hole_cards`: hero hole cards (moved by value).
    /// - `board`: current flop (three board cards).
    /// - `remaining`: deck excluding dead cards; used to generate turn/river and opponent hands.
    /// - `num_opponents`: number of opponents to enumerate (multiway enumeration used when > 1).
    ///
    /// # Returns
    ///
    /// An `EquityResult` containing counts of wins, ties, and losses accumulated across all enumerated
    /// runouts and opponent hole-card combinations.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Illustrative usage; assumes appropriate imports and a concrete evaluator implementation.
    /// let evaluator = CactusKevEvaluator::new();
    /// let calc = ExhaustiveEquityCalculator::new(evaluator);
    /// let hole = HoleCards::new(card(Rank::Ace, Suit::Spades), card(Rank::Ace, Suit::Hearts));
    /// let board = make_board(&[card(Rank::King, Suit::Spades), card(Rank::Queen, Suit::Clubs), card(Rank::Ten, Suit::Hearts)]);
    /// let remaining = Deck::new(); // deck must exclude dead cards in real use
    /// let result = calc.calculate_flop(hole, &board, &remaining, 1);
    /// assert!(result.wins + result.ties + result.losses > 0);
    /// ```
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

    /// Computes exact preflop equity by exhaustively enumerating all possible five-card boards
    /// and single-opponent hole-card combinations.
    ///
    /// For `num_opponents == 1`, this method iterates every distinct 5-card board from `remaining`
    /// and every legal opponent two-card hand, tallying wins, ties, and losses into an `EquityResult`.
    /// For `num_opponents > 1` exhaustive enumeration is computationally infeasible; the method
    /// returns an `EquityResult` with zeroed counts in that case.
    ///
    /// # Parameters
    ///
    /// - `num_opponents`: number of opponents at the table; only `1` is supported for exhaustive calculation.
    ///
    /// # Returns
    ///
    /// An `EquityResult` constructed from the aggregated win, tie, and loss counts for the provided
    /// `hole_cards` against the enumerated runouts and opponent hands.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let evaluator = CactusKevEvaluator::new();
    /// let calc = ExhaustiveEquityCalculator::new(evaluator);
    /// let hero = HoleCards::new(Card::ace_of_spades(), Card::ace_of_hearts());
    /// let deck = Deck::full().exclude_hole_cards(hero);
    /// let result = calc.calculate(hero, &Board::empty(), 1);
    /// assert!(result.equity() > 0.0);
    /// ```
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

    /// Enumerates all opponent hole-card combinations for a complete 5-card board and updates win/tie/loss counters.
    ///
    /// This function exhaustively assigns remaining unseen cards as hole cards to 2- or 3-opponent multiway scenarios,
    /// evaluates each opponent's best 7-card hand against the hero's hand, and increments the provided `wins`, `ties`,
    /// or `losses` counters for each distinct assignment. If `num_opponents` is greater than 3 the function returns
    /// immediately without modifying the counters. For `num_opponents == 1`, callers should use the single-opponent
    /// enumeration path in the caller instead of this function.
    ///
    /// Parameters:
    /// - `hole_cards`: hero's hole cards (by value).
    /// - `board`: a complete 5-card board used for all evaluations.
    /// - `remaining`: deck of unseen cards to deal to opponents.
    /// - `num_opponents`: number of opponents to enumerate; supported values for exhaustive enumeration are 2 and 3.
    /// - `wins`, `ties`, `losses`: mutable counters incremented for each opponent assignment where the hero wins,
    ///   ties, or loses respectively.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut wins = 0u64;
    /// let mut ties = 0u64;
    /// let mut losses = 0u64;
    /// // enumerate for two opponents
    /// // calc.enumerate_multiway(hole, &board, &deck, 2, &mut wins, &mut ties, &mut losses);
    /// ```
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