//! ID generation port for unique identifiers.

use std::sync::atomic::{AtomicU64, Ordering};

/// Port for generating unique identifiers.
///
/// This trait abstracts ID generation to allow:
/// - Different ID formats (UUID, sequential, custom)
/// - Deterministic testing with predictable IDs
/// - Distributed ID generation strategies
pub trait IdGenerator: Send + Sync {
    /// Generate a new unique identifier.
    fn generate(&self) -> String;
}

/// A sequential ID generator that produces incrementing numeric IDs.
///
/// Thread-safe using atomic operations. Useful for testing
/// where predictable, ordered IDs are helpful.
#[derive(Debug)]
pub struct SequentialIdGenerator {
    counter: AtomicU64,
    prefix: String,
}

impl SequentialIdGenerator {
    /// Create a new sequential generator starting from 1.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
            prefix: String::new(),
        }
    }

    /// Create a new sequential generator with a prefix.
    ///
    /// # Arguments
    /// * `prefix` - String prefix for generated IDs (e.g., "game-", "hand-")
    #[must_use]
    pub fn with_prefix(prefix: impl Into<String>) -> Self {
        Self {
            counter: AtomicU64::new(0),
            prefix: prefix.into(),
        }
    }

    /// Create a new sequential generator starting from a specific value.
    ///
    /// # Arguments
    /// * `start` - The first ID to generate
    #[must_use]
    pub const fn starting_from(start: u64) -> Self {
        Self {
            counter: AtomicU64::new(start.saturating_sub(1)),
            prefix: String::new(),
        }
    }
}

impl Default for SequentialIdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl IdGenerator for SequentialIdGenerator {
    fn generate(&self) -> String {
        let id = self.counter.fetch_add(1, Ordering::SeqCst) + 1;
        if self.prefix.is_empty() {
            id.to_string()
        } else {
            format!("{}{id}", self.prefix)
        }
    }
}

/// A UUID-based ID generator.
///
/// Generates random UUIDs (v4-like) using timestamp and random components.
/// For production use, consider using the `uuid` crate instead.
#[derive(Debug, Clone, Copy, Default)]
pub struct SimpleUuidGenerator;

impl SimpleUuidGenerator {
    /// Create a new UUID generator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl IdGenerator for SimpleUuidGenerator {
    fn generate(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        // Simple pseudo-random using timestamp bits
        let random_part = timestamp.wrapping_mul(6_364_136_223_846_793_005);

        format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            (timestamp & 0xFFFF_FFFF) as u32,
            ((timestamp >> 32) & 0xFFFF) as u16,
            ((random_part >> 48) & 0x0FFF) as u16 | 0x4000, // Version 4
            ((random_part >> 32) & 0x3FFF) as u16 | 0x8000, // Variant
            (random_part & 0xFFFF_FFFF_FFFF) as u64,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequential_generator_increments() {
        let generator = SequentialIdGenerator::new();
        assert_eq!(generator.generate(), "1");
        assert_eq!(generator.generate(), "2");
        assert_eq!(generator.generate(), "3");
    }

    #[test]
    fn test_sequential_generator_with_prefix() {
        let generator = SequentialIdGenerator::with_prefix("game-");
        assert_eq!(generator.generate(), "game-1");
        assert_eq!(generator.generate(), "game-2");
    }

    #[test]
    fn test_sequential_generator_starting_from() {
        let generator = SequentialIdGenerator::starting_from(100);
        assert_eq!(generator.generate(), "100");
        assert_eq!(generator.generate(), "101");
    }

    #[test]
    fn test_uuid_generator_format() {
        let generator = SimpleUuidGenerator::new();
        let id = generator.generate();

        // Check UUID format: 8-4-4-4-12
        let parts: Vec<&str> = id.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].len(), 8);
        assert_eq!(parts[1].len(), 4);
        assert_eq!(parts[2].len(), 4);
        assert_eq!(parts[3].len(), 4);
        assert_eq!(parts[4].len(), 12);
    }

    #[test]
    fn test_uuid_generator_uniqueness() {
        let generator = SimpleUuidGenerator::new();
        let id1 = generator.generate();
        // Small delay to ensure different timestamp
        std::thread::sleep(std::time::Duration::from_millis(1));
        let id2 = generator.generate();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_sequential_generator_thread_safety() {
        use std::sync::Arc;
        use std::thread;

        let generator = Arc::new(SequentialIdGenerator::new());
        let mut handles = vec![];

        for _ in 0..10 {
            let generator_clone = Arc::clone(&generator);
            handles.push(thread::spawn(move || {
                let mut ids = vec![];
                for _ in 0..100 {
                    ids.push(generator_clone.generate());
                }
                ids
            }));
        }

        let mut all_ids: Vec<String> = handles
            .into_iter()
            .flat_map(|h| h.join().unwrap())
            .collect();

        // All IDs should be unique
        all_ids.sort();
        let original_len = all_ids.len();
        all_ids.dedup();
        assert_eq!(all_ids.len(), original_len);
    }
}
