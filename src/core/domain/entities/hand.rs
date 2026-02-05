//! Evaluated poker hand representation.

use std::cmp::Ordering;
use std::fmt;

use super::card::Card;

/// Poker hand category.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum HandRank {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    Straight = 4,
    Flush = 5,
    FullHouse = 6,
    FourOfAKind = 7,
    StraightFlush = 8,
}

/// `HandRank` - Constructors
impl HandRank {
    /// Map a numeric hand strength (1–7462) to its corresponding poker `HandRank`.
    ///
    /// The mapping follows the standard hand-strength ranges:
    /// - 1..=10 => `StraightFlush`
    /// - 11..=166 => `FourOfAKind`
    /// - 167..=322 => `FullHouse`
    /// - 323..=1599 => `Flush`
    /// - 1600..=1609 => `Straight`
    /// - 1610..=2467 => `ThreeOfAKind`
    /// - 2468..=3325 => `TwoPair`
    /// - 3326..=6185 => `OnePair`
    /// - values above 6185 => `HighCard`
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::hand::HandRank;
    /// let rank = HandRank::from_strength(1);
    /// assert_eq!(rank, HandRank::StraightFlush);
    ///
    /// let rank = HandRank::from_strength(5000);
    /// assert_eq!(rank, HandRank::OnePair);
    /// ```
    ///
    /// # Returns
    ///
    /// `HandRank` corresponding to the provided strength.
    #[must_use] 
    pub const fn from_strength(strength: u16) -> Self {
        match strength {
            1..=10 => Self::StraightFlush,
            11..=166 => Self::FourOfAKind,
            167..=322 => Self::FullHouse,
            323..=1599 => Self::Flush,
            1600..=1609 => Self::Straight,
            1610..=2467 => Self::ThreeOfAKind,
            2468..=3325 => Self::TwoPair,
            3326..=6185 => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

/// `HandRank` - Accessors
impl HandRank {
    /// Display name for the hand rank.
    ///
    /// Returns the human-readable name for the rank (for example, "Straight Flush").
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::hand::HandRank;
    /// assert_eq!(HandRank::StraightFlush.name(), "Straight Flush");
    /// assert_eq!(HandRank::HighCard.name(), "High Card");
    /// ```
    #[must_use] 
    pub const fn name(&self) -> &'static str {
        match self {
            Self::HighCard => "High Card",
            Self::OnePair => "One Pair",
            Self::TwoPair => "Two Pair",
            Self::ThreeOfAKind => "Three of a Kind",
            Self::Straight => "Straight",
            Self::Flush => "Flush",
            Self::FullHouse => "Full House",
            Self::FourOfAKind => "Four of a Kind",
            Self::StraightFlush => "Straight Flush",
        }
    }
}

impl fmt::Display for HandRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// An evaluated poker hand.
///
/// Represents the best 5-card hand with its category and strength.
/// Lower strength values indicate stronger hands (1 = royal flush).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
    rank: HandRank,
    strength: u16,
}

/// Hand - Constructors
impl Hand {
    /// Constructs a `Hand` from five `Card`s and a numeric hand `strength`.
    ///
    /// The provided `strength` is used to determine the hand's `HandRank`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Example assumes `Card` variants and `Hand` are in scope.
    /// let cards = [Card::As, Card::Ks, Card::Qs, Card::Js, Card::Ts];
    /// let hand = Hand::new(cards, 1); // strength 1 represents a royal straight flush
    /// assert_eq!(hand.strength(), 1);
    /// assert!(hand.is_royal_flush());
    /// ```
    #[must_use]
    pub const fn new(cards: [Card; 5], strength: u16) -> Self {
        let rank = HandRank::from_strength(strength);
        Self {
            cards,
            rank,
            strength,
        }
    }
}

/// Hand - Accessors
impl Hand {
    /// Accesses the five cards comprising the hand.
    ///
    /// # Returns
    ///
    /// `&[Card; 5]` — a reference to the array of five cards that form this hand.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Given a `hand: Hand`
    /// let cards = hand.cards();
    /// assert_eq!(cards.len(), 5);
    /// ```
    #[must_use] 
    pub const fn cards(&self) -> &[Card; 5] {
        &self.cards
    }

    /// Access a card in the hand by its index (0 through 4).
    ///
    /// # Parameters
    ///
    /// - `index`: Index of the card to retrieve; valid values are `0`..=`4`.
    ///
    /// # Returns
    ///
    /// The `Card` at the specified index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is outside the range `0..=4`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let hand = Hand::new([card_a, card_k, card_q, card_j, card_t], 1);
    /// let first = hand.card(0);
    /// assert_eq!(first, card_a);
    /// ```
    #[must_use] 
    pub const fn card(&self, index: usize) -> Card {
        self.cards[index]
    }

    /// Returns the hand's category as a `HandRank` (for example, `Flush` or `Full House`).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // assuming `hand` is a previously constructed `Hand`
    /// let hand: Hand = unimplemented!();
    /// let rank = hand.rank();
    /// // `rank` is a `HandRank` value classifying the hand
    /// ```
    #[must_use] 
    pub const fn rank(&self) -> HandRank {
        self.rank
    }

    /// Returns the hand's numeric strength where `1` is the strongest and `7462` is the weakest.
    ///
    /// # Returns
    ///
    /// The strength value of the hand (`1..=7462`).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // given a `Hand` instance named `hand`
    /// let s = hand.strength();
    /// assert!(s >= 1 && s <= 7462);
    /// ```
    #[must_use] 
    pub const fn strength(&self) -> u16 {
        self.strength
    }

    /// Checks whether the hand has the given rank.
    ///
    /// # Returns
    ///
    /// `true` if the hand's rank equals `rank`, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let hand = Hand::new([Card::As, Card::Ks, Card::Qs, Card::Js, Card::Ts], 1);
    /// assert!(hand.is_rank(HandRank::StraightFlush));
    /// ```
    #[must_use] 
    pub fn is_rank(&self, rank: HandRank) -> bool {
        self.rank == rank
    }

    /// Checks whether the hand is a straight flush, counting a royal flush as a straight flush.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let hand = Hand::new([Card::As, Card::Ks, Card::Qs, Card::Js, Card::Ts], 1);
    /// assert!(hand.is_straight_flush());
    /// ```
    ///
    /// @returns `true` if the hand is a straight flush (royal flush counts), `false` otherwise.
    #[must_use] 
    pub fn is_straight_flush(&self) -> bool {
        self.rank == HandRank::StraightFlush
    }

    /// Determines whether the hand is a royal flush.
    ///
    /// # Returns
    /// `true` if the hand is a royal flush, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Construct a royal flush (Ace-high straight flush) with strength 1.
    /// let hand = Hand::new([Card::As, Card::Ks, Card::Qs, Card::Js, Card::Ts], 1);
    /// assert!(hand.is_royal_flush());
    /// ``` 
    #[must_use] 
    pub const fn is_royal_flush(&self) -> bool {
        self.strength == 1
    }

    /// Determines whether the hand's rank is Four of a Kind.
    ///
    /// # Returns
    ///
    /// `true` if the hand's rank is Four of a Kind, `false` otherwise.
    #[must_use] 
    pub fn is_four_of_a_kind(&self) -> bool {
        self.rank == HandRank::FourOfAKind
    }

    /// Determines whether this hand is a full house.
    ///
    /// # Returns
    ///
    /// `true` if the hand's rank is `FullHouse`, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Construct a hand known to map to Full House by strength range (167..=322).
    /// // `cards` can be any five `Card` values; only `strength` determines the rank here.
    /// let cards = [/* five Card values */];
    /// let hand = Hand::new(cards, 200);
    /// assert!(hand.is_full_house());
    /// ```
    #[must_use] 
    pub fn is_full_house(&self) -> bool {
        self.rank == HandRank::FullHouse
    }

    /// Determines whether the hand is a flush.
    ///
    /// # Returns
    /// `true` if the hand's rank is `Flush`, `false` otherwise.
    #[must_use] 
    pub fn is_flush(&self) -> bool {
        self.rank == HandRank::Flush
    }

    /// Check if this hand is a straight.
    #[must_use] 
    pub fn is_straight(&self) -> bool {
        self.rank == HandRank::Straight
    }

    /// Reports whether the hand is Three of a Kind.
    ///
    /// # Returns
    ///
    /// `true` if the hand's rank is Three of a Kind, `false` otherwise.
    #[must_use] 
    pub fn is_three_of_a_kind(&self) -> bool {
        self.rank == HandRank::ThreeOfAKind
    }

    /// Determines whether this hand's rank is Two Pair.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Construct a hand whose strength falls in the Two Pair range (2468..=3325)
    /// // let hand = Hand::new(cards, 3000);
    /// // assert!(hand.is_two_pair());
    /// ``` 
    #[must_use] 
    pub fn is_two_pair(&self) -> bool {
        self.rank == HandRank::TwoPair
    }

    /// Checks whether the hand's rank is One Pair.
    ///
    /// # Returns
    ///
    /// `true` if the hand's rank is `OnePair`, `false` otherwise. 
    #[must_use] 
    pub fn is_one_pair(&self) -> bool {
        self.rank == HandRank::OnePair
    }

    /// Check if this hand is high card only.
    #[must_use] 
    pub fn is_high_card(&self) -> bool {
        self.rank == HandRank::HighCard
    }
}

/// Hand - Operations
impl Hand {
    /// Determines whether this hand beats another hand by comparing their strengths.
    ///
    /// # Returns
    ///
    /// `true` if this hand's strength is less than the other's strength (lower is better), `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // given two evaluated hands `a` and `b`
    /// assert_eq!(a.beats(&b), a.strength() < b.strength());
    /// ```
    #[must_use] 
    pub const fn beats(&self, other: &Self) -> bool {
        self.strength < other.strength
    }

    /// Determine whether two hands have equal strength (i.e., tie).
    ///
    /// The tie is based solely on the hands' numeric `strength` values; suits or card order do not affect the result.
    ///
    /// # Returns
    ///
    /// `true` if both hands have the same strength, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Construct two hands with the same strength (cards elided for brevity).
    /// let h1 = Hand::new([Card::AS, Card::KS, Card::QS, Card::JS, Card::TS], 1);
    /// let h2 = Hand::new([Card::AH, Card::KH, Card::QH, Card::JH, Card::TH], 1);
    /// assert!(h1.ties(&h2));
    /// ```
    #[must_use] 
    pub const fn ties(&self, other: &Self) -> bool {
        self.strength == other.strength
    }

    /// Determines whether this hand loses to another hand.
    ///
    /// Compares evaluated strengths where a larger numeric strength represents a weaker hand.
    ///
    /// # Returns
    ///
    /// `true` if this hand's strength is greater than the other hand's strength, `false` otherwise.
    #[must_use] 
    pub const fn loses_to(&self, other: &Self) -> bool {
        self.strength > other.strength
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // Lower strength is better, so reverse the comparison
        other.strength.cmp(&self.strength)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [{}{}{}{}{}]",
            self.rank,
            self.cards[0],
            self.cards[1],
            self.cards[2],
            self.cards[3],
            self.cards[4]
        )
    }
}

impl From<([Card; 5], u16)> for Hand {
    /// Creates a Hand from a tuple containing five cards and a numeric strength.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let cards = [Card::As, Card::Ks, Card::Qs, Card::Js, Card::Ts];
    /// let hand = Hand::from((cards, 1));
    /// assert!(hand.is_royal_flush());
    /// assert_eq!(hand.strength(), 1);
    /// ```
    fn from((cards, strength): ([Card; 5], u16)) -> Self {
        Self::new(cards, strength)
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
    fn test_hand_rank_from_strength() {
        assert_eq!(HandRank::from_strength(1), HandRank::StraightFlush);
        assert_eq!(HandRank::from_strength(10), HandRank::StraightFlush);
        assert_eq!(HandRank::from_strength(11), HandRank::FourOfAKind);
        assert_eq!(HandRank::from_strength(166), HandRank::FourOfAKind);
        assert_eq!(HandRank::from_strength(167), HandRank::FullHouse);
        assert_eq!(HandRank::from_strength(322), HandRank::FullHouse);
        assert_eq!(HandRank::from_strength(323), HandRank::Flush);
        assert_eq!(HandRank::from_strength(1599), HandRank::Flush);
        assert_eq!(HandRank::from_strength(1600), HandRank::Straight);
        assert_eq!(HandRank::from_strength(1609), HandRank::Straight);
        assert_eq!(HandRank::from_strength(1610), HandRank::ThreeOfAKind);
        assert_eq!(HandRank::from_strength(2467), HandRank::ThreeOfAKind);
        assert_eq!(HandRank::from_strength(2468), HandRank::TwoPair);
        assert_eq!(HandRank::from_strength(3325), HandRank::TwoPair);
        assert_eq!(HandRank::from_strength(3326), HandRank::OnePair);
        assert_eq!(HandRank::from_strength(6185), HandRank::OnePair);
        assert_eq!(HandRank::from_strength(6186), HandRank::HighCard);
        assert_eq!(HandRank::from_strength(7462), HandRank::HighCard);
    }

    #[test]
    fn test_hand_rank_ordering() {
        assert!(HandRank::StraightFlush > HandRank::FourOfAKind);
        assert!(HandRank::FourOfAKind > HandRank::FullHouse);
        assert!(HandRank::FullHouse > HandRank::Flush);
        assert!(HandRank::Flush > HandRank::Straight);
        assert!(HandRank::Straight > HandRank::ThreeOfAKind);
        assert!(HandRank::ThreeOfAKind > HandRank::TwoPair);
        assert!(HandRank::TwoPair > HandRank::OnePair);
        assert!(HandRank::OnePair > HandRank::HighCard);
    }

    #[test]
    fn test_hand_creation() {
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Spades),
        ];
        let hand = Hand::new(cards, 1);

        assert_eq!(hand.rank(), HandRank::StraightFlush);
        assert_eq!(hand.strength(), 1);
        assert!(hand.is_royal_flush());
        assert!(hand.is_straight_flush());
    }

    #[test]
    fn test_hand_card_accessor() {
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Spades),
        ];
        let hand = Hand::new(cards, 1);

        assert_eq!(hand.card(0), cards[0]);
        assert_eq!(hand.card(4), cards[4]);
        assert_eq!(hand.cards(), &cards);
    }

    #[test]
    fn test_hand_comparison() {
        let cards1 = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Spades),
        ];
        let royal_flush = Hand::new(cards1, 1);

        let cards2 = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::Ace, Suit::Hearts),
            card(Rank::Ace, Suit::Diamonds),
            card(Rank::Ace, Suit::Clubs),
            card(Rank::King, Suit::Spades),
        ];
        let four_aces = Hand::new(cards2, 11);

        assert!(royal_flush.beats(&four_aces));
        assert!(four_aces.loses_to(&royal_flush));
        assert!(!royal_flush.ties(&four_aces));
        assert!(royal_flush > four_aces);
    }

    #[test]
    fn test_hand_ties() {
        let cards1 = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Spades),
        ];
        let hand1 = Hand::new(cards1, 1);

        let cards2 = [
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Hearts),
            card(Rank::Jack, Suit::Hearts),
            card(Rank::Ten, Suit::Hearts),
        ];
        let hand2 = Hand::new(cards2, 1);

        assert!(hand1.ties(&hand2));
        assert!(!hand1.beats(&hand2));
        assert!(!hand1.loses_to(&hand2));
    }

    #[test]
    fn test_hand_display() {
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Spades),
        ];
        let hand = Hand::new(cards, 1);
        let display = format!("{}", hand);
        assert!(display.contains("Straight Flush"));
        assert!(display.contains("As"));
    }

    #[test]
    fn test_hand_from_tuple() {
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Spades),
            card(Rank::Queen, Suit::Spades),
            card(Rank::Jack, Suit::Spades),
            card(Rank::Ten, Suit::Spades),
        ];
        let hand: Hand = (cards, 1).into();
        assert_eq!(hand.strength(), 1);
        assert!(hand.is_royal_flush());
    }

    #[test]
    fn test_hand_rank_predicates() {
        let cards = [
            card(Rank::Ace, Suit::Spades),
            card(Rank::King, Suit::Hearts),
            card(Rank::Queen, Suit::Diamonds),
            card(Rank::Jack, Suit::Clubs),
            card(Rank::Nine, Suit::Spades),
        ];

        let high_card = Hand::new(cards, 7000);
        assert!(high_card.is_high_card());
        assert!(!high_card.is_one_pair());

        let one_pair = Hand::new(cards, 4000);
        assert!(one_pair.is_one_pair());
        assert!(!one_pair.is_two_pair());

        let two_pair = Hand::new(cards, 3000);
        assert!(two_pair.is_two_pair());

        let three_kind = Hand::new(cards, 2000);
        assert!(three_kind.is_three_of_a_kind());

        let straight = Hand::new(cards, 1605);
        assert!(straight.is_straight());

        let flush = Hand::new(cards, 500);
        assert!(flush.is_flush());

        let full_house = Hand::new(cards, 200);
        assert!(full_house.is_full_house());

        let four_kind = Hand::new(cards, 50);
        assert!(four_kind.is_four_of_a_kind());

        let straight_flush = Hand::new(cards, 5);
        assert!(straight_flush.is_straight_flush());
        assert!(!straight_flush.is_royal_flush());
    }
}