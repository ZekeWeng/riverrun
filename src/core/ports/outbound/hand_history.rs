//! Hand history recording port.

use std::error::Error;
use std::fmt;

use crate::core::domain::entities::board::Board;
use crate::core::domain::entities::card::Card;
use crate::core::domain::entities::hand::Hand;

/// Unique identifier for a recorded hand.
pub type HandId = String;

/// A complete record of a played hand.
#[derive(Debug, Clone)]
pub struct HandRecord {
    /// Unique identifier for this hand.
    pub id: HandId,
    /// Number of players in the hand.
    pub num_players: usize,
    /// Hole cards for each player (indexed by player position).
    pub hole_cards: Vec<[Card; 2]>,
    /// The final board state.
    pub board: Board,
    /// Evaluated hands for each player at showdown.
    pub final_hands: Option<Vec<Hand>>,
    /// Indices of winning player(s).
    pub winners: Vec<usize>,
}

/// Error type for hand history operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandHistoryError {
    /// Failed to write the hand record.
    WriteError(String),
    /// Failed to read a hand record.
    ReadError(String),
    /// The requested hand was not found.
    NotFound(HandId),
}

impl fmt::Display for HandHistoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HandHistoryError::WriteError(msg) => write!(f, "write error: {}", msg),
            HandHistoryError::ReadError(msg) => write!(f, "read error: {}", msg),
            HandHistoryError::NotFound(id) => write!(f, "hand not found: {}", id),
        }
    }
}

impl Error for HandHistoryError {}

/// Port for recording and retrieving hand histories.
///
/// This trait defines the interface for hand history storage.
/// Implementations can:
/// - Write to log files
/// - Store in a database
/// - Send to analytics services
pub trait HandHistoryWriter: Send + Sync {
    /// Record a completed hand.
    ///
    /// # Arguments
    /// * `record` - The hand record to store
    ///
    /// # Returns
    /// `Ok(())` on success, or a `HandHistoryError` on failure.
    fn write(&self, record: &HandRecord) -> Result<(), HandHistoryError>;
}

/// Port for reading hand histories.
///
/// Separate from `HandHistoryWriter` to allow write-only implementations.
pub trait HandHistoryReader: Send + Sync {
    /// Retrieve a hand record by ID.
    ///
    /// # Arguments
    /// * `id` - The unique identifier of the hand
    ///
    /// # Returns
    /// The hand record, or a `HandHistoryError` if not found.
    fn read(&self, id: &HandId) -> Result<HandRecord, HandHistoryError>;

    /// List recent hand IDs.
    ///
    /// # Arguments
    /// * `limit` - Maximum number of IDs to return
    ///
    /// # Returns
    /// A vector of hand IDs, most recent first.
    fn list_recent(&self, limit: usize) -> Result<Vec<HandId>, HandHistoryError>;
}
