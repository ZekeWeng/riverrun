//! Outbound ports (driven ports) for external dependencies.
//!
//! These traits define interfaces that the application requires from external
//! systems. Adapters implement these traits to connect to actual infrastructure.
//!
//! # Architecture
//!

mod clock;
mod event_publisher;
mod event_store;
mod id_generator;
mod random_source;
mod read_model;
mod snapshot_store;

// Time
pub use clock::{Clock, FixedClock, SystemClock, Timestamp};

// Event Sourcing
pub use event_store::{EventStore, EventStoreError, GameId, StoredEvent, Version};
pub use snapshot_store::{Snapshot, SnapshotError, SnapshotPolicy, SnapshotStore};

// Read Models (Projections)
pub use read_model::{
    ActiveGameState, ActiveGameStore, HandId, HandSummary, HandSummaryStore, PlayerId,
    PlayerStats, PlayerStatsStore, ReadModelError,
};

// Real-time Notifications
pub use event_publisher::{GameNotification, NoOpPublisher, NotificationPublisher, Street};

// Utilities
pub use id_generator::{IdGenerator, SequentialIdGenerator, SimpleUuidGenerator};
pub use random_source::{FixedRandomSource, RandRandomSource, RandomSource};
