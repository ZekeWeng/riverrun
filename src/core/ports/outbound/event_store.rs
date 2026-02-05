//! Event store port for event sourcing.

use std::error::Error;
use std::fmt;

use super::clock::Timestamp;

/// Unique identifier for a game/aggregate.
pub type GameId = String;

/// Version number for optimistic concurrency control.
pub type Version = u64;

/// A stored event with metadata.
#[derive(Debug, Clone)]
pub struct StoredEvent<E> {
    /// The event payload.
    pub event: E,
    /// Version of this event (1-indexed, monotonically increasing per aggregate).
    pub version: Version,
    /// Timestamp when the event was stored.
    pub timestamp: Timestamp,
    /// The game/aggregate this event belongs to.
    pub game_id: GameId,
}

impl<E> StoredEvent<E> {
    /// Create a new stored event.
    pub fn new(event: E, version: Version, timestamp: Timestamp, game_id: GameId) -> Self {
        StoredEvent {
            event,
            version,
            timestamp,
            game_id,
        }
    }
}

/// Error type for event store operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventStoreError {
    /// Optimistic concurrency conflict - expected version doesn't match.
    ConcurrencyConflict {
        expected: Version,
        actual: Version,
    },
    /// The requested game/aggregate was not found.
    NotFound(GameId),
    /// A storage or I/O error occurred.
    StorageError(String),
    /// Failed to serialize/deserialize event data.
    SerializationError(String),
}

impl fmt::Display for EventStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventStoreError::ConcurrencyConflict { expected, actual } => {
                write!(
                    f,
                    "concurrency conflict: expected version {}, actual {}",
                    expected, actual
                )
            }
            EventStoreError::NotFound(id) => write!(f, "game not found: {}", id),
            EventStoreError::StorageError(msg) => write!(f, "storage error: {}", msg),
            EventStoreError::SerializationError(msg) => write!(f, "serialization error: {}", msg),
        }
    }
}

impl Error for EventStoreError {}

/// Port for storing and retrieving domain events.
///
/// This trait defines the interface for event sourcing storage.
/// Events are immutable once stored and form the source of truth.
///
/// # Type Parameter
/// * `E` - The event type to store (typically an enum of all domain events)
pub trait EventStore<E>: Send + Sync {
    /// Append events to an aggregate's event stream.
    ///
    /// # Arguments
    /// * `game_id` - The aggregate/game identifier
    /// * `events` - Events to append
    /// * `expected_version` - Expected current version for optimistic concurrency.
    ///   Use 0 for new aggregates, or the version of the last event read.
    ///
    /// # Returns
    /// The new version after appending, or an error if:
    /// - `ConcurrencyConflict`: Another write happened since we read
    /// - `StorageError`: I/O or database error
    fn append(
        &self,
        game_id: &GameId,
        events: Vec<E>,
        expected_version: Version,
    ) -> Result<Version, EventStoreError>;

    /// Load all events for an aggregate.
    ///
    /// # Arguments
    /// * `game_id` - The aggregate/game identifier
    ///
    /// # Returns
    /// All stored events in order, or `NotFound` if the aggregate doesn't exist.
    fn load(&self, game_id: &GameId) -> Result<Vec<StoredEvent<E>>, EventStoreError>;

    /// Load events starting from a specific version.
    ///
    /// Useful for catching up on events after reconnection.
    ///
    /// # Arguments
    /// * `game_id` - The aggregate/game identifier
    /// * `from_version` - Load events with version > from_version
    ///
    /// # Returns
    /// Events after the specified version, or empty vec if none.
    fn load_from(
        &self,
        game_id: &GameId,
        from_version: Version,
    ) -> Result<Vec<StoredEvent<E>>, EventStoreError>;

    /// Get the current version (number of events) for an aggregate.
    ///
    /// # Arguments
    /// * `game_id` - The aggregate/game identifier
    ///
    /// # Returns
    /// The current version, or 0 if the aggregate doesn't exist.
    fn version(&self, game_id: &GameId) -> Result<Version, EventStoreError>;

    /// Check if an aggregate exists.
    ///
    /// # Arguments
    /// * `game_id` - The aggregate/game identifier
    fn exists(&self, game_id: &GameId) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stored_event_creation() {
        let event = StoredEvent::new("test_event", 1, 1000, "game-1".to_string());
        assert_eq!(event.event, "test_event");
        assert_eq!(event.version, 1);
        assert_eq!(event.timestamp, 1000);
        assert_eq!(event.game_id, "game-1");
    }

    #[test]
    fn test_event_store_error_display() {
        let err = EventStoreError::ConcurrencyConflict {
            expected: 5,
            actual: 7,
        };
        assert_eq!(
            err.to_string(),
            "concurrency conflict: expected version 5, actual 7"
        );

        let err = EventStoreError::NotFound("game-123".to_string());
        assert_eq!(err.to_string(), "game not found: game-123");

        let err = EventStoreError::StorageError("connection failed".to_string());
        assert_eq!(err.to_string(), "storage error: connection failed");

        let err = EventStoreError::SerializationError("invalid format".to_string());
        assert_eq!(err.to_string(), "serialization error: invalid format");
    }
}
