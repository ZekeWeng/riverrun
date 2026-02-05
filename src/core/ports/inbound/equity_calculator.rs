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
    /// Create an EquityResult from raw win/tie/loss counts and the number of opponents.
    ///
    /// Computes win, tie, and loss rates as fractions of the total samples. Tie equity is
    /// divided evenly among all tied players (opponents + 1) and added to the win rate to
    /// produce the overall equity. If the total sample count is zero, all rates, equity,
    /// and samples are set to zero.
    ///
    /// # Parameters
    ///
    /// - `wins` — number of samples where the hand won outright.
    /// - `ties` — number of samples that resulted in a tie (split pot).
    /// - `losses` — number of samples where the hand lost.
    /// - `num_opponents` — number of opposing players (excludes the hand being evaluated).
    ///
    /// # Returns
    ///
    /// An `EquityResult` with the computed `equity`, `win_rate`, `tie_rate`, `lose_rate`, and
    /// `samples` (the total number of provided counts).
    ///
    /// # Examples
    ///
    /// ```
    /// let res = EquityResult::from_counts(10, 2, 3, 2);
    /// assert_eq!(res.samples(), 15);
    /// // equity == win_rate + tie_rate / (num_opponents + 1)
    /// ```
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
    /// Overall equity as a fraction between 0.0 and 1.0.
    ///
    /// The value represents the probability of winning or receiving a share of tied outcomes
    /// against the specified number of opponents.
    ///
    /// # Examples
    ///
    /// ```
    /// let r = EquityResult::from_counts(60, 20, 20, 1);
    /// let e = r.equity();
    /// assert!(e >= 0.0 && e <= 1.0);
    /// ```
    #[must_use] 
    pub const fn equity(&self) -> f64 {
        self.equity
    }

    /// Convert the stored equity to a percentage between 0 and 100.
    ///
    /// # Returns
    ///
    /// `equity` expressed as a percentage between 0 and 100.
    ///
    /// # Examples
    ///
    /// ```
    /// let res = EquityResult::from_counts(1, 0, 3, 1); // 1 win, 0 ties, 3 losses => equity = 0.25
    /// let pct = res.equity_percent();
    /// assert!((pct - 25.0).abs() < 1e-12);
    /// ```
    #[must_use] 
    pub fn equity_percent(&self) -> f64 {
        self.equity * 100.0
    }

    /// The pure win rate from the computed equity results.
    ///
    /// This returns the fraction of evaluated samples that were wins, expressed as a
    /// value between 0.0 and 1.0.
    ///
    /// # Examples
    ///
    /// ```
    /// let r = EquityResult::from_counts(1, 0, 1, 1);
    /// assert_eq!(r.win_rate(), 0.5);
    /// ```
    #[must_use] 
    pub const fn win_rate(&self) -> f64 {
        self.win_rate
    }

    /// Win rate expressed as a percentage (0.0–100.0).
    ///
    /// # Examples
    ///
    /// ```
    /// let r = EquityResult::from_counts(30, 10, 60, 1);
    /// assert!((r.win_percent() - r.win_rate() * 100.0).abs() < 1e-12);
    /// ```
    #[must_use] 
    pub fn win_percent(&self) -> f64 {
        self.win_rate * 100.0
    }

    /// The tie rate as a fraction of evaluated samples.
    ///
    /// # Returns
    /// A `f64` in the range `[0.0, 1.0]` representing the proportion of samples that resulted in a tie.
    ///
    /// # Examples
    ///
    /// ```
    /// let res = EquityResult::from_counts(1, 1, 1, 1);
    /// assert!(res.tie_rate() >= 0.0 && res.tie_rate() <= 1.0);
    /// ```
    #[must_use] 
    pub const fn tie_rate(&self) -> f64 {
        self.tie_rate
    }

    /// Represents the tie rate as a percentage between 0 and 100.
    ///
    /// # Examples
    ///
    /// ```
    /// let res = EquityResult { equity: 0.0, win_rate: 0.0, tie_rate: 0.25, lose_rate: 0.0, samples: 0 };
    /// assert_eq!(res.tie_percent(), 25.0);
    /// ```
    #[must_use] 
    pub fn tie_percent(&self) -> f64 {
        self.tie_rate * 100.0
    }

    /// Returns the proportion of evaluated samples that resulted in a loss.
    ///
    /// The value is a fraction between 0.0 and 1.0 where 1.0 means all samples lost.
    ///
    /// # Examples
    ///
    /// ```
    /// let r = EquityResult::from_counts(0, 0, 10, 1);
    /// assert_eq!(r.lose_rate(), 1.0);
    /// ```
    #[must_use] 
    pub const fn lose_rate(&self) -> f64 {
        self.lose_rate
    }

    /// Lose rate expressed as a percentage (0 to 100).
    ///
    /// # Returns
    ///
    /// `f64` — lose rate as a percentage in the range 0.0 to 100.0.
    ///
    /// # Examples
    ///
    /// ```
    /// let r = EquityResult { equity: 0.0, win_rate: 0.2, tie_rate: 0.1, lose_rate: 0.7, samples: 100 };
    /// assert_eq!(r.lose_percent(), 70.0);
    /// ```
    #[must_use] 
    pub fn lose_percent(&self) -> f64 {
        self.lose_rate * 100.0
    }

    /// Number of evaluated samples used to produce the equity and rates.
    ///
    /// This returns the raw count of matchups or simulations that contributed to the
    /// `EquityResult`.
    ///
    /// # Examples
    ///
    /// ```
    /// let res = EquityResult::from_counts(42, 0, 0, 1);
    /// assert_eq!(res.samples(), 42);
    /// ```
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