//! Game state persistence port.

use std::error::Error;
use std::fmt;

use crate::core::domain::entities::game::Game;

/// Unique identifier for a persisted game.
pub type GameId = String;

/// Error type for repository operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryError {
    /// The requested game was not found.
    NotFound(GameId),
    /// A storage or I/O error occurred.
    StorageError(String),
    /// The data format was invalid or corrupted.
    InvalidData(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::NotFound(id) => write!(f, "game not found: {}", id),
            RepositoryError::StorageError(msg) => write!(f, "storage error: {}", msg),
            RepositoryError::InvalidData(msg) => write!(f, "invalid data: {}", msg),
        }
    }
}

impl Error for RepositoryError {}

/// Port for persisting and retrieving game state.
///
/// This trait defines the interface for game storage.
/// Implementations can use various backends:
/// - In-memory (for testing)
/// - File system
/// - Database
/// - Remote service
pub trait GameRepository: Send + Sync {
    /// Save a game to storage.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the game
    /// * `game` - The game state to persist
    ///
    /// # Returns
    /// `Ok(())` on success, or a `RepositoryError` on failure.
    fn save(&self, id: &GameId, game: &Game) -> Result<(), RepositoryError>;

    /// Load a game from storage.
    ///
    /// # Arguments
    /// * `id` - Unique identifier of the game to load
    ///
    /// # Returns
    /// The game state, or a `RepositoryError` if not found or corrupted.
    fn load(&self, id: &GameId) -> Result<Game, RepositoryError>;

    /// Delete a game from storage.
    ///
    /// # Arguments
    /// * `id` - Unique identifier of the game to delete
    ///
    /// # Returns
    /// `Ok(())` on success (including if game didn't exist), or a `RepositoryError` on failure.
    fn delete(&self, id: &GameId) -> Result<(), RepositoryError>;

    /// Check if a game exists in storage.
    ///
    /// # Arguments
    /// * `id` - Unique identifier to check
    ///
    /// # Returns
    /// `true` if the game exists, `false` otherwise.
    fn exists(&self, id: &GameId) -> bool;
}
