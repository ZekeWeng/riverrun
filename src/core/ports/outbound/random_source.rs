//! Random number generation port for deck shuffling and dealing.

use crate::core::domain::entities::card::Card;

/// Port for random number generation.
///
/// This trait abstracts random number generation to allow:
/// - Dependency injection of different RNG implementations
/// - Deterministic testing with seeded RNGs
/// - Swapping RNG strategies (cryptographic vs fast)
pub trait RandomSource: Send + Sync {
    /// Shuffle a slice of cards in place.
    ///
    /// # Arguments
    /// * `cards` - Mutable slice of cards to shuffle
    fn shuffle_cards(&mut self, cards: &mut [Card]);

    /// Generate a random index in the range [0, max).
    ///
    /// # Arguments
    /// * `max` - Exclusive upper bound
    ///
    /// # Returns
    /// A random index in [0, max)
    ///
    /// # Panics
    /// May panic if max is 0.
    fn random_index(&mut self, max: usize) -> usize;
}

/// A deterministic "random" source that always returns fixed values.
///
/// Useful for testing scenarios where predictable behavior is needed.
#[derive(Debug, Clone)]
pub struct FixedRandomSource {
    index: usize,
}

impl FixedRandomSource {
    /// Create a new fixed random source.
    ///
    /// Always returns the same index (clamped to valid range).
    #[must_use]
    pub const fn new(fixed_index: usize) -> Self {
        Self { index: fixed_index }
    }

    /// Create a fixed source that always returns 0.
    #[must_use]
    pub const fn zero() -> Self {
        Self { index: 0 }
    }
}

impl RandomSource for FixedRandomSource {
    fn shuffle_cards(&mut self, _cards: &mut [Card]) {
        // Do nothing - cards remain in original order
    }

    fn random_index(&mut self, max: usize) -> usize {
        if max == 0 {
            0
        } else {
            self.index % max
        }
    }
}

/// A wrapper around `rand::Rng` to implement `RandomSource`.
pub struct RandRandomSource<R: rand::Rng> {
    rng: R,
}

impl<R: rand::Rng> RandRandomSource<R> {
    /// Create a new random source wrapping the given RNG.
    pub const fn new(rng: R) -> Self {
        Self { rng }
    }

    /// Get a reference to the underlying RNG.
    #[must_use]
    pub const fn inner(&self) -> &R {
        &self.rng
    }

    /// Get a mutable reference to the underlying RNG.
    pub fn inner_mut(&mut self) -> &mut R {
        &mut self.rng
    }
}

impl<R: rand::Rng + Send + Sync> RandomSource for RandRandomSource<R> {
    fn shuffle_cards(&mut self, cards: &mut [Card]) {
        use rand::seq::SliceRandom;
        cards.shuffle(&mut self.rng);
    }

    fn random_index(&mut self, max: usize) -> usize {
        self.rng.random_range(0..max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::entities::card::{Rank, Suit};

    fn make_cards() -> Vec<Card> {
        vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Clubs),
        ]
    }

    #[test]
    fn test_fixed_random_source_no_shuffle() {
        let mut source = FixedRandomSource::new(0);
        let mut cards = make_cards();
        let original = cards.clone();

        source.shuffle_cards(&mut cards);

        // Cards should remain in original order
        assert_eq!(cards, original);
    }

    #[test]
    fn test_fixed_random_source_index() {
        let mut source = FixedRandomSource::new(5);

        assert_eq!(source.random_index(10), 5);
        assert_eq!(source.random_index(10), 5);
        assert_eq!(source.random_index(3), 2); // 5 % 3 = 2
    }

    #[test]
    fn test_fixed_random_source_zero() {
        let mut source = FixedRandomSource::zero();

        assert_eq!(source.random_index(10), 0);
        assert_eq!(source.random_index(100), 0);
    }

    #[test]
    fn test_fixed_random_source_zero_max() {
        let mut source = FixedRandomSource::new(5);
        assert_eq!(source.random_index(0), 0);
    }

    #[test]
    fn test_rand_random_source_shuffle() {
        use rand::SeedableRng;
        let rng = rand::rngs::StdRng::seed_from_u64(42);
        let mut source = RandRandomSource::new(rng);

        let mut cards = make_cards();
        let original = cards.clone();

        source.shuffle_cards(&mut cards);

        // With a seeded RNG, shuffle should change the order
        // (statistically very unlikely to stay the same)
        assert_ne!(cards, original);
    }

    #[test]
    fn test_rand_random_source_deterministic() {
        use rand::SeedableRng;

        // Same seed should produce same results
        let rng1 = rand::rngs::StdRng::seed_from_u64(42);
        let rng2 = rand::rngs::StdRng::seed_from_u64(42);

        let mut source1 = RandRandomSource::new(rng1);
        let mut source2 = RandRandomSource::new(rng2);

        let mut cards1 = make_cards();
        let mut cards2 = make_cards();

        source1.shuffle_cards(&mut cards1);
        source2.shuffle_cards(&mut cards2);

        assert_eq!(cards1, cards2);
    }

    #[test]
    fn test_rand_random_source_index() {
        use rand::SeedableRng;
        let rng = rand::rngs::StdRng::seed_from_u64(42);
        let mut source = RandRandomSource::new(rng);

        // Should produce values in valid range
        for _ in 0..100 {
            let idx = source.random_index(10);
            assert!(idx < 10);
        }
    }

    #[test]
    fn test_rand_random_source_inner() {
        use rand::SeedableRng;
        let rng = rand::rngs::StdRng::seed_from_u64(42);
        let mut source = RandRandomSource::new(rng);

        // Can access inner RNG
        let _ = source.inner();
        let _ = source.inner_mut();
    }
}
