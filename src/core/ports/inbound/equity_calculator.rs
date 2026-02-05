//! Equity calculation port for poker hand analysis.

use crate::core::domain::entities::board::Board;
use crate::core::domain::entities::hole_cards::HoleCards;

/// Result of an equity calculation.
#[derive(Debug, Clone, Copy)]
pub struct EquityResult {
    equity: f64,
    win_rate: f64,
    tie_rate: f64,
    lose_rate: f64,
    samples: u64,
}

/// `EquityResult` - Constructors
impl EquityResult {
    /// Create a new equity result from raw counts.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn from_counts(wins: u64, ties: u64, losses: u64, num_opponents: usize) -> Self {
        let total = wins + ties + losses;
        if total == 0 {
            return Self {
                equity: 0.0,
                win_rate: 0.0,
                tie_rate: 0.0,
                lose_rate: 0.0,
                samples: 0,
            };
        }

        let total_f = total as f64;
        let win_rate = wins as f64 / total_f;
        let tie_rate = ties as f64 / total_f;
        let lose_rate = losses as f64 / total_f;

        // In a tie, equity is split among tying players
        let tie_share = tie_rate / (num_opponents + 1) as f64;
        let equity = win_rate + tie_share;

        Self {
            equity,
            win_rate,
            tie_rate,
            lose_rate,
            samples: total,
        }
    }
}

/// `EquityResult` - Accessors
impl EquityResult {
    /// Get overall equity (probability of winning or getting a share of ties).
    #[must_use] 
    pub const fn equity(&self) -> f64 {
        self.equity
    }

    /// Get equity as a percentage (0-100).
    #[must_use] 
    pub fn equity_percent(&self) -> f64 {
        self.equity * 100.0
    }

    /// Get pure win rate.
    #[must_use] 
    pub const fn win_rate(&self) -> f64 {
        self.win_rate
    }

    /// Get win rate as a percentage (0-100).
    #[must_use] 
    pub fn win_percent(&self) -> f64 {
        self.win_rate * 100.0
    }

    /// Get tie rate.
    #[must_use] 
    pub const fn tie_rate(&self) -> f64 {
        self.tie_rate
    }

    /// Get tie rate as a percentage (0-100).
    #[must_use] 
    pub fn tie_percent(&self) -> f64 {
        self.tie_rate * 100.0
    }

    /// Get lose rate.
    #[must_use] 
    pub const fn lose_rate(&self) -> f64 {
        self.lose_rate
    }

    /// Get lose rate as a percentage (0-100).
    #[must_use] 
    pub fn lose_percent(&self) -> f64 {
        self.lose_rate * 100.0
    }

    /// Get the number of samples/matchups evaluated.
    #[must_use] 
    pub const fn samples(&self) -> u64 {
        self.samples
    }
}

impl std::fmt::Display for EquityResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Equity: {:.2}% (W: {:.2}%, T: {:.2}%, L: {:.2}%) [{} samples]",
            self.equity_percent(),
            self.win_percent(),
            self.tie_percent(),
            self.lose_percent(),
            self.samples
        )
    }
}

/// Port for calculating poker hand equity.
///
/// Equity represents the probability of winning (plus share of ties)
/// against opponent hand(s) given the current board state.
pub trait EquityCalculator: Send + Sync {
    /// Calculate equity against random opponent hands using default settings.
    ///
    /// # Arguments
    /// * `hole_cards` - Your hole cards
    /// * `board` - Current community cards (can be empty for preflop)
    /// * `num_opponents` - Number of opponents (1 for heads-up)
    ///
    /// # Returns
    /// An `EquityResult` containing win/tie/lose percentages.
    fn calculate(&self, hole_cards: &HoleCards, board: &Board, num_opponents: usize)
        -> EquityResult;

    /// Calculate equity with a specific number of samples/iterations.
    ///
    /// For exhaustive calculators, this may be ignored.
    /// For Monte Carlo calculators, this controls the number of simulations.
    ///
    /// # Arguments
    /// * `hole_cards` - Your hole cards
    /// * `board` - Current community cards
    /// * `num_opponents` - Number of opponents
    /// * `samples` - Number of samples/iterations to run
    fn calculate_sampled(
        &self,
        hole_cards: &HoleCards,
        board: &Board,
        num_opponents: usize,
        samples: u32,
    ) -> EquityResult;
}
