//! Game event publishing port for real-time notifications.

use super::clock::Timestamp;
use super::event_store::GameId;
use super::read_model::PlayerId;

/// Events that can be published for real-time notification.
///
/// These are simplified events for external consumers (UI, webhooks).
/// Full event details are stored in the event store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameNotification {
    /// A new game has started.
    GameStarted {
        game_id: GameId,
        timestamp: Timestamp,
        num_players: usize,
        player_ids: Vec<PlayerId>,
    },

    /// A player joined the game.
    PlayerJoined {
        game_id: GameId,
        timestamp: Timestamp,
        player_id: PlayerId,
    },

    /// Hole cards have been dealt.
    HoleCardsDealt {
        game_id: GameId,
        timestamp: Timestamp,
    },

    /// Community cards dealt (flop/turn/river).
    StreetDealt {
        game_id: GameId,
        timestamp: Timestamp,
        street: Street,
    },

    /// The hand has reached showdown.
    Showdown {
        game_id: GameId,
        timestamp: Timestamp,
        winner_ids: Vec<PlayerId>,
    },

    /// The game has ended.
    GameEnded {
        game_id: GameId,
        timestamp: Timestamp,
    },
}

/// Street enum for notifications (separate from domain to avoid coupling).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Street {
    Preflop,
    Flop,
    Turn,
    River,
}

impl GameNotification {
    /// Get the game ID from any notification.
    pub fn game_id(&self) -> &GameId {
        match self {
            GameNotification::GameStarted { game_id, .. }
            | GameNotification::PlayerJoined { game_id, .. }
            | GameNotification::HoleCardsDealt { game_id, .. }
            | GameNotification::StreetDealt { game_id, .. }
            | GameNotification::Showdown { game_id, .. }
            | GameNotification::GameEnded { game_id, .. } => game_id,
        }
    }

    /// Get the timestamp from any notification.
    pub fn timestamp(&self) -> Timestamp {
        match self {
            GameNotification::GameStarted { timestamp, .. }
            | GameNotification::PlayerJoined { timestamp, .. }
            | GameNotification::HoleCardsDealt { timestamp, .. }
            | GameNotification::StreetDealt { timestamp, .. }
            | GameNotification::Showdown { timestamp, .. }
            | GameNotification::GameEnded { timestamp, .. } => *timestamp,
        }
    }
}

/// Port for publishing game notifications in real-time.
///
/// Implementations can:
/// - Send over WebSocket to connected clients
/// - Publish to a message queue (Kafka, RabbitMQ)
/// - Trigger webhooks
/// - Log for debugging
pub trait NotificationPublisher: Send + Sync {
    /// Publish a notification.
    fn publish(&self, notification: GameNotification);

    /// Publish multiple notifications in order.
    fn publish_batch(&self, notifications: &[GameNotification]) {
        for notification in notifications {
            self.publish(notification.clone());
        }
    }
}

/// A no-op publisher that discards all notifications.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoOpPublisher;

impl NotificationPublisher for NoOpPublisher {
    fn publish(&self, _notification: GameNotification) {
        // Intentionally empty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_game_id() {
        let notification = GameNotification::GameStarted {
            game_id: "game-123".to_string(),
            timestamp: 1000,
            num_players: 4,
            player_ids: vec![],
        };
        assert_eq!(notification.game_id(), "game-123");
    }

    #[test]
    fn test_notification_timestamp() {
        let notification = GameNotification::StreetDealt {
            game_id: "game-123".to_string(),
            timestamp: 2000,
            street: Street::Flop,
        };
        assert_eq!(notification.timestamp(), 2000);
    }

    #[test]
    fn test_noop_publisher() {
        let publisher = NoOpPublisher;
        // Should not panic
        publisher.publish(GameNotification::GameEnded {
            game_id: "game-1".to_string(),
            timestamp: 0,
        });
    }
}
