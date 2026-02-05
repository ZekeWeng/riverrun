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
    /// Returns a slice of the winner indices in order, limited to `winner_count`.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = ShowdownResult { winners: [3, 5, 0, 0, 0, 0, 0, 0, 0, 0], winner_count: 2 };
    /// assert_eq!(result.winner_indices(), &[3, 5]);
    /// ```
    #[must_use] 
    pub fn winner_indices(&self) -> &[usize] {
        &self.winners[..self.winner_count]
    }

    /// Returns whether the result represents exactly one winner.
    ///
    /// # Examples
    ///
    /// ```
    /// let res = ShowdownResult { winners: [0; MAX_PLAYERS], winner_count: 1 };
    /// assert!(res.is_single_winner());
    /// ```
    #[must_use] 
    pub const fn is_single_winner(&self) -> bool {
        self.winner_count == 1
    }

    /// Indicates whether the showdown resulted in multiple winners.
    ///
    /// # Examples
    ///
    /// ```
    /// // construct a ShowdownResult with two winners
    /// let res = ShowdownResult { winners: [0usize; MAX_PLAYERS], winner_count: 2 };
    /// assert!(res.is_tie());
    /// ```
    #[must_use] 
    pub const fn is_tie(&self) -> bool {
        self.winner_count > 1
    }

    /// Returns the single winner's player index when there is exactly one winner.
    ///
    /// # Returns
    ///
    /// `Some(index)` if exactly one winner is present, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let res = ShowdownResult { winners: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0], winner_count: 1 };
    /// assert_eq!(res.single_winner(), Some(2));
    /// ```
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
    /// Returns a slice of the winner indices in order, limited to `winner_count`.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = ShowdownResult { winners: [3, 5, 0, 0, 0, 0, 0, 0, 0, 0], winner_count: 2 };
    /// assert_eq!(result.winner_indices(), &[3, 5]);
    /// ```
    #[must_use] 
    pub fn winner_indices(&self) -> &[usize] {
        &self.winners[..self.winner_count]
    }

    /// Returns whether the result represents exactly one winner.
    ///
    /// # Examples
    ///
    /// ```
    /// let res = ShowdownResult { winners: [0; MAX_PLAYERS], winner_count: 1 };
    /// assert!(res.is_single_winner());
    /// ```
    #[must_use] 
    pub const fn is_single_winner(&self) -> bool {
        self.winner_count == 1
    }

    /// Indicates whether the showdown resulted in multiple winners.
    ///
    /// # Examples
    ///
    /// ```
    /// // construct a ShowdownResult with two winners
    /// let res = ShowdownResult { winners: [0usize; MAX_PLAYERS], winner_count: 2 };
    /// assert!(res.is_tie());
    /// ```
    #[must_use] 
    pub const fn is_tie(&self) -> bool {
        self.winner_count > 1
    }

    /// Returns the single winner's player index when there is exactly one winner.
    ///
    /// # Returns
    ///
    /// `Some(index)` if exactly one winner is present, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let res = ShowdownResult { winners: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0], winner_count: 1 };
    /// assert_eq!(res.single_winner(), Some(2));
    /// ```
    #[must_use] 
    pub const fn single_winner(&self) -> Option<usize> {
        if self.winner_count == 1 {
            Some(self.winners[0])
        } else {
            None
        }
    }

    /// Get references to the evaluated hand(s) belonging to the winning player(s).
    ///
    /// The returned vector contains references to the `Hand` instances for each index
    /// reported by `winner_indices()`, in the same order.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let winners = result.winning_hands();
    /// for hand in winners {
    ///     // inspect each winning hand
    ///     println!("{:?}", hand);
    /// }
    /// ```
    #[must_use] 
    pub fn winning_hands(&self) -> Vec<&Hand> {
        self.winner_indices()
            .iter()
            .map(|&idx| &self.hands[idx])
            .collect()
    }

    /// Get the evaluated hand for the player at the given zero-based index, if present.
    ///
    /// # Parameters
    /// - `player_idx` — zero-based index of the player whose hand to retrieve.
    ///
    /// # Returns
    /// `Some(&Hand)` with the player's evaluated hand when present, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Assuming `result` is a ShowdownResultWithHands:
    /// // let result: ShowdownResultWithHands = ...;
    /// // let maybe_hand = result.hand(0);
    /// // assert!(maybe_hand.is_none() || maybe_hand.is_some());
    /// ```
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