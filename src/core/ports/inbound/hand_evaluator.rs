use crate::core::domain::entities::card::Card;
use crate::core::domain::entities::hand::Hand;

/// Port for evaluating poker hands.
///
/// This trait defines the interface for hand evaluation algorithms.
/// Implementations can use different strategies (Cactus Kev, Two Plus Two, etc.)
/// while consumers remain decoupled from the specific algorithm.
pub trait HandEvaluator: Send + Sync {
    /// Evaluate a 5-card hand and return the full Hand entity.
    ///
    /// # Arguments
    /// * `cards` - Exactly 5 cards to evaluate
    ///
    /// # Returns
    /// A `Hand` containing the cards, rank category, and strength.
    fn evaluate_5cards(&self, cards: [Card; 5]) -> Hand;

    /// Evaluate a 7-card hand (Texas Hold'em) and return the best 5-card Hand.
    ///
    /// # Arguments
    /// * `cards` - Exactly 7 cards (2 hole cards + 5 board cards)
    ///
    /// # Returns
    /// The best possible 5-card `Hand` from the 7 cards.
    fn evaluate_7cards(&self, cards: [Card; 7]) -> Hand;

    /// Evaluate a 5-card hand and return only the numeric strength.
    ///
    /// This is a performance optimization for cases where only the
    /// strength is needed (e.g., Monte Carlo simulations).
    ///
    /// # Returns
    /// A rank where lower is better (1 = royal flush, 7462 = worst high card).
    fn evaluate_5cards_fast(&self, cards: &[Card; 5]) -> u16;

    /// Evaluate a 7-card hand and return only the numeric strength.
    ///
    /// This is a performance optimization for cases where only the
    /// strength is needed.
    ///
    /// # Returns
    /// The best possible 5-card hand strength from the 7 cards.
    fn evaluate_7cards_fast(&self, cards: &[Card; 7]) -> u16;
}
