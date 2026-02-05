use crate::core::domain::entities::board::Board;
use crate::core::domain::entities::hand::Hand;
use crate::core::domain::entities::hole_cards::HoleCards;

/// Maximum number of players supported in a hand.
pub const MAX_PLAYERS: usize = 10;

/// Result of solving a poker hand showdown.
#[derive(Debug, Clone)]
pub struct ShowdownResult {
    /// Indices of winning player(s). Multiple indices if tied.
    pub winners: [usize; MAX_PLAYERS],
    /// Number of winners (1 for single winner, >1 for ties).
    pub winner_count: usize,
}

/// `ShowdownResult` - Accessors
impl ShowdownResult {
    /// Get the winner indices as a slice.
    #[must_use] 
    pub fn winner_indices(&self) -> &[usize] {
        &self.winners[..self.winner_count]
    }

    /// Check if there is a single winner.
    #[must_use] 
    pub const fn is_single_winner(&self) -> bool {
        self.winner_count == 1
    }

    /// Check if there is a tie (multiple winners).
    #[must_use] 
    pub const fn is_tie(&self) -> bool {
        self.winner_count > 1
    }

    /// Get the single winner index, if there is exactly one winner.
    #[must_use] 
    pub const fn single_winner(&self) -> Option<usize> {
        if self.winner_count == 1 {
            Some(self.winners[0])
        } else {
            None
        }
    }
}

/// Extended result including each player's evaluated hand.
#[derive(Debug, Clone)]
pub struct ShowdownResultWithHands {
    /// Indices of winning player(s).
    pub winners: [usize; MAX_PLAYERS],
    /// Number of winners.
    pub winner_count: usize,
    /// Evaluated hand for each player.
    pub hands: Vec<Hand>,
}

/// `ShowdownResultWithHands` - Accessors
impl ShowdownResultWithHands {
    /// Get the winner indices as a slice.
    #[must_use] 
    pub fn winner_indices(&self) -> &[usize] {
        &self.winners[..self.winner_count]
    }

    /// Check if there is a single winner.
    #[must_use] 
    pub const fn is_single_winner(&self) -> bool {
        self.winner_count == 1
    }

    /// Check if there is a tie (multiple winners).
    #[must_use] 
    pub const fn is_tie(&self) -> bool {
        self.winner_count > 1
    }

    /// Get the single winner index, if there is exactly one winner.
    #[must_use] 
    pub const fn single_winner(&self) -> Option<usize> {
        if self.winner_count == 1 {
            Some(self.winners[0])
        } else {
            None
        }
    }

    /// Get the winning hand(s).
    #[must_use] 
    pub fn winning_hands(&self) -> Vec<&Hand> {
        self.winner_indices()
            .iter()
            .map(|&idx| &self.hands[idx])
            .collect()
    }

    /// Get a player's hand by index.
    #[must_use] 
    pub fn hand(&self, player_idx: usize) -> Option<&Hand> {
        self.hands.get(player_idx)
    }
}

/// Port for determining the winner(s) of a poker hand.
///
/// This trait defines the interface for showdown resolution.
/// Implementations handle comparing multiple players' hands and
/// determining winner(s) including tie scenarios.
pub trait HandSolver: Send + Sync {
    /// Determine the winner(s) of a Texas Hold'em hand.
    ///
    /// # Arguments
    /// * `players` - Slice of hole cards for each player
    /// * `board` - The community board (must be complete with 5 cards)
    ///
    /// # Returns
    /// A `ShowdownResult` containing winner indices and count.
    ///
    /// # Panics
    /// Panics if the board is not complete (doesn't have 5 cards).
    fn solve(&self, players: &[HoleCards], board: &Board) -> ShowdownResult;

    /// Determine the winner(s) and evaluated hands of a Texas Hold'em hand.
    ///
    /// # Arguments
    /// * `players` - Slice of hole cards for each player
    /// * `board` - The community board (must be complete with 5 cards)
    ///
    /// # Returns
    /// A `ShowdownResultWithHands` containing winners and each player's full Hand.
    ///
    /// # Panics
    /// Panics if the board is not complete (doesn't have 5 cards).
    fn solve_with_hands(&self, players: &[HoleCards], board: &Board) -> ShowdownResultWithHands;
}
