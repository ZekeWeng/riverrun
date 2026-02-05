//! Snapshot store port for event sourcing optimization.
//!
//! Snapshots store the aggregate state at a point in time to avoid
//! replaying all events when loading an aggregate.

use std::error::Error;
use std::fmt;

use super::clock::Timestamp;
use super::event_store::{GameId, Version};

/// Error type for snapshot store operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotError {
    /// The requested snapshot was not found.
    NotFound(GameId),
    /// A storage or I/O error occurred.
    StorageError(String),
    /// Failed to serialize/deserialize snapshot data.
    SerializationError(String),
}

impl fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(id) => write!(f, "snapshot not found: {id}"),
            Self::StorageError(msg) => write!(f, "storage error: {msg}"),
            Self::SerializationError(msg) => write!(f, "serialization error: {msg}"),
        }
    }
}

impl Error for SnapshotError {}

/// A stored snapshot with metadata.
#[derive(Debug, Clone)]
pub struct Snapshot<S> {
    /// The snapshot state.
    pub state: S,
    /// The event version this snapshot was taken at.
    pub version: Version,
    /// Timestamp when the snapshot was taken.
    pub timestamp: Timestamp,
    /// The game/aggregate this snapshot belongs to.
    pub game_id: GameId,
}

impl<S> Snapshot<S> {
    /// Create a new snapshot.
    #[must_use]
    pub const fn new(state: S, version: Version, timestamp: Timestamp, game_id: GameId) -> Self {
        Self {
            state,
            version,
            timestamp,
            game_id,
        }
    }
}

/// Port for storing and retrieving aggregate snapshots.
///
/// Snapshots are an optimization for event sourcing. Instead of replaying
/// all events from the beginning, we can load a snapshot and only replay
/// events that occurred after the snapshot.
///
/// # Type Parameter
/// * `S` - The snapshot state type (typically the aggregate state)
pub trait SnapshotStore<S>: Send + Sync {
    /// Save a snapshot of the aggregate state.
    ///
    /// # Arguments
    /// * `snapshot` - The snapshot to store
    ///
    /// # Errors
    /// Returns `SnapshotError::StorageError` on I/O errors.
    /// Returns `SnapshotError::SerializationError` if serialization fails.
    fn save(&self, snapshot: &Snapshot<S>) -> Result<(), SnapshotError>;

    /// Load the most recent snapshot for an aggregate.
    ///
    /// # Arguments
    /// * `game_id` - The aggregate/game identifier
    ///
    /// # Errors
    /// Returns `SnapshotError::NotFound` if no snapshot exists.
    /// Returns `SnapshotError::StorageError` on I/O errors.
    fn load(&self, game_id: &GameId) -> Result<Snapshot<S>, SnapshotError>;

    /// Delete all snapshots for an aggregate.
    ///
    /// # Arguments
    /// * `game_id` - The aggregate/game identifier
    ///
    /// # Errors
    /// Returns `SnapshotError::StorageError` on I/O errors.
    fn delete(&self, game_id: &GameId) -> Result<(), SnapshotError>;

    /// Check if a snapshot exists for an aggregate.
    fn exists(&self, game_id: &GameId) -> bool;
}

/// Configuration for when to take snapshots.
#[derive(Debug, Clone, Copy)]
pub struct SnapshotPolicy {
    /// Take a snapshot every N events.
    pub every_n_events: u64,
    /// Take a snapshot if more than N events since last snapshot.
    pub max_events_since_snapshot: u64,
}

impl Default for SnapshotPolicy {
    fn default() -> Self {
        Self {
            every_n_events: 100,
            max_events_since_snapshot: 100,
        }
    }
}

impl SnapshotPolicy {
    /// Check if a snapshot should be taken.
    ///
    /// # Arguments
    /// * `current_version` - Current event version
    /// * `last_snapshot_version` - Version of the last snapshot (0 if none)
    #[must_use]
    pub const fn should_snapshot(
        &self,
        current_version: Version,
        last_snapshot_version: Version,
    ) -> bool {
        let events_since = current_version.saturating_sub(last_snapshot_version);
        events_since >= self.max_events_since_snapshot
            || (current_version > 0 && current_version.is_multiple_of(self.every_n_events))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_creation() {
        let snapshot = Snapshot::new("test_state", 10, 1000, "game-1".to_string());
        assert_eq!(snapshot.state, "test_state");
        assert_eq!(snapshot.version, 10);
        assert_eq!(snapshot.timestamp, 1000);
        assert_eq!(snapshot.game_id, "game-1");
    }

    #[test]
    fn test_snapshot_error_display() {
        let err = SnapshotError::NotFound("game-123".to_string());
        assert_eq!(err.to_string(), "snapshot not found: game-123");

        let err = SnapshotError::StorageError("disk full".to_string());
        assert_eq!(err.to_string(), "storage error: disk full");

        let err = SnapshotError::SerializationError("invalid format".to_string());
        assert_eq!(err.to_string(), "serialization error: invalid format");
    }

    #[test]
    fn test_snapshot_policy_default() {
        let policy = SnapshotPolicy::default();
        assert_eq!(policy.every_n_events, 100);
        assert_eq!(policy.max_events_since_snapshot, 100);
    }

    #[test]
    fn test_snapshot_policy_should_snapshot() {
        let policy = SnapshotPolicy {
            every_n_events: 10,
            max_events_since_snapshot: 15,
        };

        // No snapshot at version 0
        assert!(!policy.should_snapshot(0, 0));

        // Snapshot at version 10 (every_n_events)
        assert!(policy.should_snapshot(10, 0));

        // No snapshot at version 11 (only 1 event since snapshot at 10)
        assert!(!policy.should_snapshot(11, 10));

        // Snapshot at version 25 (15 events since snapshot at 10)
        assert!(policy.should_snapshot(25, 10));

        // Snapshot at version 20 (every_n_events)
        assert!(policy.should_snapshot(20, 15));
    }
}
