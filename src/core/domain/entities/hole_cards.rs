//! Hole cards representation for Texas Hold'em.

use super::card::Card;

/// A player's two private hole cards.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HoleCards {
    cards: [Card; 2],
}

/// `HoleCards` - Constructors
impl HoleCards {
    /// Constructs a HoleCards containing two private cards in the given order.
    ///
    /// # Examples
    ///
    /// ```
    /// // Create two Card values (constructor shown illustratively).
    /// let a = Card::new(Rank::Ace, Suit::Spades);
    /// let k = Card::new(Rank::King, Suit::Hearts);
    /// let hc = HoleCards::new(a, k);
    /// assert_eq!(hc.first(), a);
    /// assert_eq!(hc.second(), k);
    /// ```
    #[must_use] 
    pub const fn new(first: Card, second: Card) -> Self {
        Self {
            cards: [first, second],
        }
    }
}

/// `HoleCards` - Accessors
impl HoleCards {
    /// Return the first card of the hole cards.
    ///
    /// # Examples
    ///
    /// ```
    /// let c1 = Card::new_rank_suit(Rank::Ace, Suit::Spades);
    /// let c2 = Card::new_rank_suit(Rank::King, Suit::Hearts);
    /// let hc = HoleCards::new(c1, c2);
    /// assert_eq!(hc.first(), c1);
    /// ```
    #[must_use] 
    pub const fn first(&self) -> Card {
        self.cards[0]
    }

    /// Accesses the player's second hole card.
    ///
    /// # Examples
    ///
    /// ```
    /// let hc = HoleCards::new(Card::AceSpades, Card::KingHearts);
    /// assert_eq!(hc.second(), Card::KingHearts);
    /// ```
    #[must_use] 
    pub const fn second(&self) -> Card {
        self.cards[1]
    }

    /// Returns a reference to the two hole cards stored in this `HoleCards`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let hc = HoleCards::from([card1, card2]);
    /// assert_eq!(hc.cards(), &[card1, card2]);
    /// ```
    #[must_use] 
    pub const fn cards(&self) -> &[Card; 2] {
        &self.cards
    }

    /// Determine whether the two hole cards share the same suit.
    ///
    /// # Returns
    ///
    /// `true` if both hole cards have the same suit, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::{Card, HoleCards, Rank, Suit};
    ///
    /// let hc = HoleCards::new(
    ///     Card::new(Rank::Ace, Suit::Spades),
    ///     Card::new(Rank::King, Suit::Spades),
    /// );
    /// assert!(hc.is_suited());
    /// ```
    #[must_use]
    pub const fn is_suited(&self) -> bool {
        self.cards[0].same_suit(&self.cards[1])
    }

    /// Check if hole cards are a pocket pair (same rank).
    #[must_use]
    pub const fn is_pair(&self) -> bool {
        self.cards[0].same_rank(&self.cards[1])
    }

    /// Computes the rank gap between the two hole cards (0 = connected, 1 = one-gapper, etc.).
    ///
    /// The result is the number of distinct ranks between the two cards when measured around the rank
    /// sequence (Ace and Two count as connected). Values are bounded to the valid rank span and fit in
    /// a `u8`.
    ///
    /// # Returns
    ///
    /// `u8` gap between the hole cards: `0` for adjacent ranks, `1` for one-gap, and so on.
    pub fn gap(&self) -> u8 {
        let combined = self.cards[0].rank_bits() | self.cards[1].rank_bits();
        let span = (31 - combined.leading_zeros()) - combined.trailing_zeros();
        span.min(13 - span).saturating_sub(1) as u8
    }

    /// Determines whether the two hole cards have consecutive ranks (connected).
    ///
    /// # Returns
    ///
    /// `true` if the two cards are consecutive in rank (for example, 7-8 or Ace-2), `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// // Example (constructors may vary in your crate):
    /// let hc = HoleCards::new(Card::ace_of_spades(), Card::two_of_hearts());
    /// assert!(hc.is_connected());
    /// ```
    #[must_use] 
    pub fn is_connected(&self) -> bool {
        self.gap() == 0
    }

    /// Produces a 7-card array by appending a 5-card board to these hole cards.
    ///
    /// # Examples
    ///
    /// ```
    /// // construct hole cards and board (types and constructors depend on crate)
    /// let hole = HoleCards::new(Card::from_str("As"), Card::from_str("Kh"));
    /// let board = [
    ///     Card::from_str("2c"),
    ///     Card::from_str("3d"),
    ///     Card::from_str("4h"),
    ///     Card::from_str("5s"),
    ///     Card::from_str("6c"),
    /// ];
    /// let combined = hole.combine_with_board(board);
    /// assert_eq!(combined[0], hole.first());
    /// assert_eq!(combined[1], hole.second());
    /// assert_eq!(combined[2..], board);
    /// ```
    #[must_use] 
    pub const fn combine_with_board(&self, board: [Card; 5]) -> [Card; 7] {
        [
            self.cards[0],
            self.cards[1],
            board[0],
            board[1],
            board[2],
            board[3],
            board[4],
        ]
    }
}

impl std::fmt::Display for HoleCards {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.cards[0], self.cards[1])
    }
}

impl From<[Card; 2]> for HoleCards {
    /// Creates a `HoleCards` value from an array of two `Card` values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let hole = HoleCards::from([first_card, second_card]);
    /// assert_eq!(hole.first(), first_card);
    /// assert_eq!(hole.second(), second_card);
    /// ```
    fn from(cards: [Card; 2]) -> Self {
        Self { cards }
    }
}

impl From<(Card, Card)> for HoleCards {
    /// Create `HoleCards` from a `(Card, Card)` tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// let hole = HoleCards::from((first_card, second_card));
    /// assert_eq!(hole.first(), first_card);
    /// ```
    fn from((first, second): (Card, Card)) -> Self {
        Self::new(first, second)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::entities::card::{Rank, Suit};

    fn card(rank: Rank, suit: Suit) -> Card {
        Card::new(rank, suit)
    }

    #[test]
    fn test_new_hole_cards() {
        let hole = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
        );
        assert_eq!(hole.first(), card(Rank::Ace, Suit::Spades));
        assert_eq!(hole.second(), card(Rank::King, Suit::Spades));
    }

    #[test]
    fn test_is_suited() {
        let suited = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
        );
        let offsuit = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
        );
        assert!(suited.is_suited());
        assert!(!offsuit.is_suited());
    }

    #[test]
    fn test_is_pair() {
        let pair = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
        );
        let not_pair = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
        );
        assert!(pair.is_pair());
        assert!(!not_pair.is_pair());
    }

    #[test]
    fn test_is_connected() {
        let connected = HoleCards::new(
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Hearts),
        );
        let ace_two = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::Two, Suit::Hearts),
        );
        let gapped = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::Jack, Suit::Hearts),
        );
        assert!(connected.is_connected());
        assert!(ace_two.is_connected());
        assert!(!gapped.is_connected());
    }

    #[test]
    fn test_gap() {
        let connected = HoleCards::new(
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Hearts),
        );
        let one_gap = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::Queen, Suit::Hearts),
        );
        let two_gap = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::Jack, Suit::Hearts),
        );
        assert_eq!(connected.gap(), 0);
        assert_eq!(one_gap.gap(), 1);
        assert_eq!(two_gap.gap(), 2);
    }

    #[test]
    fn test_display() {
        let hole = HoleCards::new(
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
        );
        assert_eq!(hole.to_string(), "AsKh");
    }

    #[test]
    fn test_from_array() {
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
        ];
        let hole: HoleCards = cards.into();
        assert_eq!(hole.first(), cards[0]);
    }

    #[test]
    fn test_from_tuple() {
        let hole: HoleCards = (
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
        ).into();
        assert_eq!(hole.to_string(), "AsKh");
    }
}