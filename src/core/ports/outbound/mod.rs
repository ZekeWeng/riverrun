//! Outbound ports (driven ports) for external dependencies.
//!
//! These traits define interfaces that the application requires from external
//! systems. Adapters implement these traits to connect to actual infrastructure.

mod event_publisher;
mod game_repository;
mod hand_history;
mod random_source;

pub use event_publisher::{EventPublisher, GameEvent, NoOpPublisher};
pub use game_repository::{GameId, GameRepository, RepositoryError};
pub use hand_history::{HandHistoryError, HandHistoryReader, HandHistoryWriter, HandId, HandRecord};
pub use random_source::RandomSource;
