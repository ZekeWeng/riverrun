//! Game event publishing port.

use crate::core::domain::entities::board::Street;
use crate::core::domain::entities::card::Card;

/// Events that can occur during a poker game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameEvent {
    /// A new game has started.
    GameStarted {
        /// Number of players in the game.
        num_players: usize,
    },

    /// Hole cards have been dealt to players.
    HoleCardsDealt {
        /// Number of players who received cards.
        num_players: usize,
    },

    /// Community cards have been dealt.
    CommunityCardsDealt {
        /// The current street after dealing.
        street: Street,
        /// The cards that were just dealt.
        cards: Vec<Card>,
    },

    /// The game has reached showdown.
    Showdown {
        /// Indices of the winning player(s).
        winners: Vec<usize>,
    },

    /// The game has been reset for a new hand.
    GameReset,
}

/// Port for publishing game events.
///
/// This trait defines the interface for event notification.
/// Implementations can:
/// - Log events to a file or console
/// - Send events over a network (WebSocket, message queue)
/// - Trigger UI updates
/// - Record analytics
pub trait EventPublisher: Send + Sync {
    /// Publish a game event.
    ///
    /// # Arguments
    /// * `event` - The event to publish
    fn publish(&self, event: GameEvent);

    /// Publish multiple events in order.
    ///
    /// Default implementation calls `publish` for each event.
    ///
    /// # Arguments
    /// * `events` - The events to publish
    fn publish_batch(&self, events: &[GameEvent]) {
        for event in events {
            self.publish(event.clone());
        }
    }
}

/// A no-op event publisher that discards all events.
///
/// Useful for testing or when event publishing is not needed.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoOpPublisher;

impl EventPublisher for NoOpPublisher {
    fn publish(&self, _event: GameEvent) {
        // Intentionally empty - discards all events
    }
}
