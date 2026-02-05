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
    fn random_index(&mut self, max: usize) -> usize;
}
