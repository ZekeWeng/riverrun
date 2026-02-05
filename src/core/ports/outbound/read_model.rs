//! Read model ports for queryable projections.
//!
//! Read models are denormalized views of data optimized for specific queries.
//! They are built by projecting events from the event store.

use std::error::Error;
use std::fmt;

use super::clock::Timestamp;
use super::event_store::GameId;

/// Unique identifier for a player.
pub type PlayerId = String;

/// Unique identifier for a hand record.
pub type HandId = String;

/// Error type for read model operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadModelError {
    /// The requested record was not found.
    NotFound(String),
    /// A storage or I/O error occurred.
    StorageError(String),
    /// Query parameters were invalid.
    InvalidQuery(String),
}

impl fmt::Display for ReadModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadModelError::NotFound(id) => write!(f, "not found: {}", id),
            ReadModelError::StorageError(msg) => write!(f, "storage error: {}", msg),
            ReadModelError::InvalidQuery(msg) => write!(f, "invalid query: {}", msg),
        }
    }
}

impl Error for ReadModelError {}

/// Summary of a completed hand for querying.
///
/// This is a denormalized read model built from events.
#[derive(Debug, Clone)]
pub struct HandSummary {
    /// Unique identifier for this hand.
    pub hand_id: HandId,
    /// The game this hand belongs to.
    pub game_id: GameId,
    /// Timestamp when the hand started.
    pub started_at: Timestamp,
    /// Timestamp when the hand ended.
    pub ended_at: Timestamp,
    /// Number of players in the hand.
    pub num_players: usize,
    /// Player IDs who participated.
    pub player_ids: Vec<PlayerId>,
    /// Player IDs who won.
    pub winner_ids: Vec<PlayerId>,
    /// Whether the hand ended in a tie.
    pub is_tie: bool,
    /// The winning hand rank (e.g., "Full House", "Flush").
    pub winning_hand_rank: Option<String>,
}

impl HandSummary {
    /// Get the duration of the hand in milliseconds.
    pub fn duration_ms(&self) -> u64 {
        self.ended_at.saturating_sub(self.started_at)
    }
}

/// Port for storing and querying hand summaries.
pub trait HandSummaryStore: Send + Sync {
    /// Save or update a hand summary.
    fn save(&self, summary: &HandSummary) -> Result<(), ReadModelError>;

    /// Get a hand summary by ID.
    fn get(&self, hand_id: &HandId) -> Result<HandSummary, ReadModelError>;

    /// Find hands by player ID.
    fn find_by_player(
        &self,
        player_id: &PlayerId,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<HandSummary>, ReadModelError>;

    /// Find hands within a time range.
    fn find_by_time_range(
        &self,
        from: Timestamp,
        to: Timestamp,
        limit: usize,
    ) -> Result<Vec<HandSummary>, ReadModelError>;

    /// Find hands by game ID.
    fn find_by_game(&self, game_id: &GameId) -> Result<Vec<HandSummary>, ReadModelError>;

    /// Count hands for a player.
    fn count_by_player(&self, player_id: &PlayerId) -> Result<u64, ReadModelError>;
}

/// Player statistics read model.
#[derive(Debug, Clone, Default)]
pub struct PlayerStats {
    /// The player's ID.
    pub player_id: PlayerId,
    /// Total hands played.
    pub hands_played: u64,
    /// Total hands won.
    pub hands_won: u64,
    /// Timestamp of last hand played.
    pub last_played_at: Option<Timestamp>,
}

impl PlayerStats {
    /// Calculate win rate as a percentage.
    pub fn win_rate(&self) -> f64 {
        if self.hands_played == 0 {
            0.0
        } else {
            (self.hands_won as f64 / self.hands_played as f64) * 100.0
        }
    }
}

/// Port for storing and querying player statistics.
pub trait PlayerStatsStore: Send + Sync {
    /// Get stats for a player, creating default if not exists.
    fn get(&self, player_id: &PlayerId) -> Result<PlayerStats, ReadModelError>;

    /// Update player stats (typically called when processing events).
    fn save(&self, stats: &PlayerStats) -> Result<(), ReadModelError>;

    /// Get top players by wins.
    fn top_by_wins(&self, limit: usize) -> Result<Vec<PlayerStats>, ReadModelError>;

    /// Get top players by win rate (minimum hands required).
    fn top_by_win_rate(
        &self,
        min_hands: u64,
        limit: usize,
    ) -> Result<Vec<PlayerStats>, ReadModelError>;
}

/// Active game state read model for quick lookups.
#[derive(Debug, Clone)]
pub struct ActiveGameState {
    /// The game ID.
    pub game_id: GameId,
    /// Current number of players.
    pub num_players: usize,
    /// Player IDs in the game.
    pub player_ids: Vec<PlayerId>,
    /// Current street (preflop, flop, turn, river).
    pub current_street: String,
    /// Whether the game is at showdown.
    pub is_showdown: bool,
    /// Last update timestamp.
    pub updated_at: Timestamp,
}

/// Port for storing and querying active game states.
pub trait ActiveGameStore: Send + Sync {
    /// Save or update active game state.
    fn save(&self, state: &ActiveGameState) -> Result<(), ReadModelError>;

    /// Get active game state.
    fn get(&self, game_id: &GameId) -> Result<ActiveGameState, ReadModelError>;

    /// Remove a game (when completed).
    fn remove(&self, game_id: &GameId) -> Result<(), ReadModelError>;

    /// List all active games.
    fn list_active(&self) -> Result<Vec<ActiveGameState>, ReadModelError>;

    /// Count active games.
    fn count_active(&self) -> Result<u64, ReadModelError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_summary_duration() {
        let summary = HandSummary {
            hand_id: "hand-1".to_string(),
            game_id: "game-1".to_string(),
            started_at: 1000,
            ended_at: 3500,
            num_players: 4,
            player_ids: vec![],
            winner_ids: vec![],
            is_tie: false,
            winning_hand_rank: None,
        };
        assert_eq!(summary.duration_ms(), 2500);
    }

    #[test]
    fn test_player_stats_win_rate() {
        let stats = PlayerStats {
            player_id: "player-1".to_string(),
            hands_played: 100,
            hands_won: 25,
            last_played_at: Some(1000),
        };
        assert!((stats.win_rate() - 25.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_player_stats_win_rate_zero_hands() {
        let stats = PlayerStats::default();
        assert!((stats.win_rate() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_read_model_error_display() {
        let err = ReadModelError::NotFound("hand-123".to_string());
        assert_eq!(err.to_string(), "not found: hand-123");

        let err = ReadModelError::StorageError("connection failed".to_string());
        assert_eq!(err.to_string(), "storage error: connection failed");

        let err = ReadModelError::InvalidQuery("limit must be > 0".to_string());
        assert_eq!(err.to_string(), "invalid query: limit must be > 0");
    }
}
