//!  Showdown solver that determines winner(s) of a poker hand.
//!
//! This solver uses a `HandEvaluator` to evaluate each player's hand
//! and determine the winner(s) / ties.

use crate::core::domain::entities::board::Board;
use crate::core::domain::entities::hand::Hand;
use crate::core::domain::entities::hole_cards::HoleCards;
use crate::core::ports::inbound::{
    HandEvaluator, HandSolver, ShowdownResult, ShowdownResultWithHands, MAX_PLAYERS,
};
pub struct ShowdownSolver<E: HandEvaluator> {
    evaluator: E,
}

/// ShowdownSolver - Constructors
impl<E: HandEvaluator> ShowdownSolver<E> {
    /// Create a new solver with the given hand evaluator.
    pub fn new(evaluator: E) -> Self {
        ShowdownSolver { evaluator }
    }
}

/// ShowdownSolver - Accessors
impl<E: HandEvaluator> ShowdownSolver<E> {
    /// Get a reference to the underlying evaluator.
    pub fn evaluator(&self) -> &E {
        &self.evaluator
    }
}

impl<E: HandEvaluator> HandSolver for ShowdownSolver<E> {
    fn solve(&self, players: &[HoleCards], board: &Board) -> ShowdownResult {
        let board_cards = board
            .as_array()
            .expect("Board must be complete (5 cards) for showdown");

        let mut best_strength = u16::MAX;
        let mut winners = [0usize; MAX_PLAYERS];
        let mut winner_count = 0;

        for (player_idx, hole_cards) in players.iter().enumerate() {
            let seven_cards = hole_cards.combine_with_board(board_cards);
            let strength = self.evaluator.evaluate_7cards_fast(&seven_cards);

            if strength < best_strength {
                best_strength = strength;
                winners[0] = player_idx;
                winner_count = 1;
            } else if strength == best_strength {
                winners[winner_count] = player_idx;
                winner_count += 1;
            }
        }

        ShowdownResult {
            winners,
            winner_count,
        }
    }

    fn solve_with_hands(&self, players: &[HoleCards], board: &Board) -> ShowdownResultWithHands {
        let board_cards = board
            .as_array()
            .expect("Board must be complete (5 cards) for showdown");

        let mut best_strength = u16::MAX;
        let mut winners = [0usize; MAX_PLAYERS];
        let mut winner_count = 0;
        let mut hands: Vec<Hand> = Vec::with_capacity(players.len());

        for (player_idx, hole_cards) in players.iter().enumerate() {
            let seven_cards = hole_cards.combine_with_board(board_cards);
            let hand = self.evaluator.evaluate_7cards(seven_cards);
            let strength = hand.strength();
            hands.push(hand);

            if strength < best_strength {
                best_strength = strength;
                winners[0] = player_idx;
                winner_count = 1;
            } else if strength == best_strength {
                winners[winner_count] = player_idx;
                winner_count += 1;
            }
        }

        ShowdownResultWithHands {
            winners,
            winner_count,
            hands,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::entities::card::{Card, Rank, Suit};
    use crate::core::domain::services::evaluation::CactusKevEvaluator;

    fn card(rank: Rank, suit: Suit) -> Card {
        Card::new(rank, suit)
    }

    fn make_board(cards: Vec<Card>) -> Board {
        Board::with_cards(cards).unwrap()
    }

    #[test]
    fn test_single_winner() {
        let solver = ShowdownSolver::new(CactusKevEvaluator::new());

        let players = vec![
            HoleCards::new(card(Rank::Ace, Suit::Spades), card(Rank::King, Suit::Spades)),
            HoleCards::new(card(Rank::Two, Suit::Hearts), card(Rank::Three, Suit::Hearts)),
        ];

        let board = make_board(vec![
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Diamonds),
            card(Rank::Jack, Suit::Clubs),
            card(Rank::Nine, Suit::Spades),
        ]);

        let result = solver.solve(&players, &board);
        assert!(result.is_single_winner());
        assert_eq!(result.single_winner(), Some(0));
        assert_eq!(result.winner_indices(), &[0]);
    }

    #[test]
    fn test_tie() {
        let solver = ShowdownSolver::new(CactusKevEvaluator::new());

        // Both players have the same straight from the board
        let players = vec![
            HoleCards::new(card(Rank::Two, Suit::Spades), card(Rank::Three, Suit::Spades)),
            HoleCards::new(card(Rank::Two, Suit::Hearts), card(Rank::Three, Suit::Hearts)),
        ];

        let board = make_board(vec![
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::King, Suit::Clubs),
            card(Rank::Queen, Suit::Hearts),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Diamonds),
        ]);

        let result = solver.solve(&players, &board);
        assert!(result.is_tie());
        assert_eq!(result.winner_count, 2);
        assert!(result.single_winner().is_none());
    }

    #[test]
    fn test_with_hands() {
        let solver = ShowdownSolver::new(CactusKevEvaluator::new());

        let players = vec![
            HoleCards::new(card(Rank::Ace, Suit::Spades), card(Rank::King, Suit::Spades)),
            HoleCards::new(card(Rank::Two, Suit::Hearts), card(Rank::Three, Suit::Hearts)),
        ];

        let board = make_board(vec![
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Diamonds),
            card(Rank::Jack, Suit::Clubs),
            card(Rank::Nine, Suit::Spades),
        ]);

        let result = solver.solve_with_hands(&players, &board);
        assert_eq!(result.hands.len(), 2);

        let player0_hand = result.hand(0).unwrap();
        let player1_hand = result.hand(1).unwrap();

        assert!(player0_hand.is_two_pair()); // AK two pair
        assert!(player0_hand.beats(player1_hand));

        let winning_hands = result.winning_hands();
        assert_eq!(winning_hands.len(), 1);
        assert!(winning_hands[0].is_two_pair());
    }

    #[test]
    fn test_result_accessors() {
        let solver = ShowdownSolver::new(CactusKevEvaluator::new());

        let players = vec![
            HoleCards::new(card(Rank::Ace, Suit::Spades), card(Rank::King, Suit::Spades)),
            HoleCards::new(card(Rank::Ace, Suit::Hearts), card(Rank::King, Suit::Hearts)),
        ];

        // Board gives both players the same straight
        let board = make_board(vec![
            card(Rank::Queen, Suit::Diamonds),
            card(Rank::Jack, Suit::Clubs),
            card(Rank::Ten, Suit::Spades),
            card(Rank::Nine, Suit::Diamonds),
            card(Rank::Two, Suit::Clubs),
        ]);

        let result = solver.solve_with_hands(&players, &board);

        // Both have A-high straight
        assert!(result.is_tie());
        assert_eq!(result.winner_indices().len(), 2);

        let winning_hands = result.winning_hands();
        assert_eq!(winning_hands.len(), 2);
        assert!(winning_hands[0].is_straight());
        assert!(winning_hands[1].is_straight());
        assert!(winning_hands[0].ties(winning_hands[1]));
    }
}
