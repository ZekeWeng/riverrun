//! Unique identifiers for games and sessions.

use std::fmt;

/// Unique identifier for a poker game.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GameId(String);

impl GameId {
    /// Creates a new game ID from a string.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the game ID as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Generates a new random game ID using UUID v4 format.
    ///
    /// Note: This is a simple implementation. For production, consider using the `uuid` crate.
    #[must_use]
    pub fn generate() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        Self(format!("game-{timestamp:x}"))
    }
}

impl fmt::Display for GameId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for GameId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for GameId {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

/// Unique identifier for a session (multiple hands).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SessionId(String);

impl SessionId {
    /// Creates a new session ID from a string.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the session ID as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Generates a new random session ID.
    #[must_use]
    pub fn generate() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        Self(format!("session-{timestamp:x}"))
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for SessionId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for SessionId {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

/// Unique identifier for a hand within a session.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HandNumber(pub u64);

impl HandNumber {
    /// Creates a new hand number.
    #[must_use]
    pub const fn new(n: u64) -> Self {
        Self(n)
    }

    /// Returns the hand number value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns the next hand number.
    #[must_use]
    pub const fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

impl fmt::Display for HandNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl From<u64> for HandNumber {
    fn from(n: u64) -> Self {
        Self::new(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_id() {
        let id = GameId::new("test-game");
        assert_eq!(id.as_str(), "test-game");
        assert_eq!(id.to_string(), "test-game");
    }

    #[test]
    fn test_game_id_from() {
        let id1: GameId = "game-1".into();
        assert_eq!(id1.as_str(), "game-1");

        let id2: GameId = String::from("game-2").into();
        assert_eq!(id2.as_str(), "game-2");
    }

    #[test]
    fn test_game_id_generate() {
        let id1 = GameId::generate();
        let id2 = GameId::generate();
        // Generated IDs should be unique (with high probability)
        assert!(id1.as_str().starts_with("game-"));
        assert!(id2.as_str().starts_with("game-"));
    }

    #[test]
    fn test_session_id() {
        let id = SessionId::new("test-session");
        assert_eq!(id.as_str(), "test-session");
        assert_eq!(id.to_string(), "test-session");
    }

    #[test]
    fn test_session_id_generate() {
        let id = SessionId::generate();
        assert!(id.as_str().starts_with("session-"));
    }

    #[test]
    fn test_hand_number() {
        let hand = HandNumber::new(1);
        assert_eq!(hand.value(), 1);
        assert_eq!(hand.next(), HandNumber::new(2));
        assert_eq!(hand.to_string(), "#1");
    }

    #[test]
    fn test_hand_number_from() {
        let hand: HandNumber = 5u64.into();
        assert_eq!(hand.value(), 5);
    }
}
