//! Card representation with Cactus Kev bit encoding.
//!
//! Encoding:
//! +--------+--------+--------+--------+
//! |xxxbbbbb|bbbbbbbb|cdhsrrrr|pppppppp|
//! +--------+--------+--------+--------+
//!
//! p = prime number for rank (2-41)        [bits 0-7]
//! r = rank (0-12, deuce to ace)           [bits 8-11]
//! cdhs = suit bits (one bit per suit)     [bits 12-15]
//! b = bit representing rank (for flush)   [bits 16-28]
//! x = unused                              [bits 29-31]
//!
//! Suit and rank are represented as enums.

use std::fmt;
use std::str::FromStr;

/// 13 prime numbers mapped to card ranks (2-A).
pub const PRIMES: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

/// Rank characters for display.
const RANK_CHARS: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

/// Suit characters for display.
const SUIT_CHARS: [char; 4] = ['c', 'd', 'h', 's'];

/// Card suit.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Suit {
    Clubs = 0,
    Diamonds = 1,
    Hearts = 2,
    Spades = 3,
}

/// Suit - Constructors
impl Suit {
    /// Convert a numeric value (0–3) to its corresponding suit.
    ///
    /// Returns `Some(Suit)` for 0 => Clubs, 1 => Diamonds, 2 => Hearts, 3 => Spades; returns `None` for any other value.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::Suit;
    /// assert_eq!(Suit::from_u8(0), Some(Suit::Clubs));
    /// assert_eq!(Suit::from_u8(3), Some(Suit::Spades));
    /// assert_eq!(Suit::from_u8(4), None);
    /// ``` 
    #[must_use] 
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Clubs),
            1 => Some(Self::Diamonds),
            2 => Some(Self::Hearts),
            3 => Some(Self::Spades),
            _ => None,
        }
    }
}

/// Suit - Accessors
impl Suit {
    /// Get the display character for this suit.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::Suit;
    /// let c = Suit::Spades.as_char();
    /// assert_eq!(c, 's');
    /// ``` 
    #[must_use] 
    pub const fn as_char(self) -> char {
        SUIT_CHARS[self as usize]
    }

    /// Get the bit mask for this suit in the Cactus Kev card encoding.
    ///
    /// The mask has a single bit set at position (`suit_index` + 12).
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::Suit;
    /// assert_eq!(Suit::Clubs.bit_mask(), 1u32 << 12);
    /// assert_eq!(Suit::Spades.bit_mask(), 1u32 << 15);
    /// ```
    #[must_use] 
    pub const fn bit_mask(self) -> u32 {
        1u32 << (self as u8 + 12)
    }

    /// Returns an iterator over the four suits in order: Clubs, Diamonds, Hearts, Spades.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::Suit;
    /// let suits: Vec<_> = Suit::all().collect();
    /// assert_eq!(suits.len(), 4);
    /// assert_eq!(suits[0], Suit::Clubs);
    /// assert_eq!(suits[3], Suit::Spades);
    /// ```
    pub fn all() -> impl Iterator<Item = Self> {
        [Self::Clubs, Self::Diamonds, Self::Hearts, Self::Spades].into_iter()
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl FromStr for Suit {
    type Err = ParseCardError;

    /// Parses a single-character suit code into a `Suit`.
    ///
    /// Accepts the characters `c`, `d`, `h`, `s` in either lower- or upper-case and returns the matching `Suit`.
    /// Returns `Err(ParseCardError::InvalidSuit)` if the input is not exactly one character or is not a recognized suit.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use riverrun::core::domain::entities::card::Suit;
    /// let s = Suit::from_str("s").unwrap();
    /// assert_eq!(s, Suit::Spades);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(ParseCardError::InvalidSuit);
        }
        match s.chars().next().unwrap() {
            'c' | 'C' => Ok(Self::Clubs),
            'd' | 'D' => Ok(Self::Diamonds),
            'h' | 'H' => Ok(Self::Hearts),
            's' | 'S' => Ok(Self::Spades),
            _ => Err(ParseCardError::InvalidSuit),
        }
    }
}

/// Card rank (2-A).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Rank {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

/// Rank - Constructors
impl Rank {
    /// Convert an integer in the range 0–12 into the corresponding `Rank`.
    ///
    /// Returns `Some(rank)` for values 0 through 12 (mapping 0→Two, …, 12→Ace), and `None` for any other value.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::Rank;
    /// assert_eq!(Rank::from_u8(0), Some(Rank::Two));
    /// assert_eq!(Rank::from_u8(12), Some(Rank::Ace));
    /// assert_eq!(Rank::from_u8(13), None);
    /// ```
    #[must_use] 
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Two),
            1 => Some(Self::Three),
            2 => Some(Self::Four),
            3 => Some(Self::Five),
            4 => Some(Self::Six),
            5 => Some(Self::Seven),
            6 => Some(Self::Eight),
            7 => Some(Self::Nine),
            8 => Some(Self::Ten),
            9 => Some(Self::Jack),
            10 => Some(Self::Queen),
            11 => Some(Self::King),
            12 => Some(Self::Ace),
            _ => None,
        }
    }
}

/// Rank - Accessors
impl Rank {
    /// Provides the single-character display symbol for the rank.
    ///
    /// # Returns
    ///
    /// The ASCII character used to display the rank (for example, `'A'` for Ace, `'T'` for Ten).
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::Rank;
    /// let c = Rank::Ace.as_char();
    /// assert_eq!(c, 'A');
    /// ```
    #[must_use] 
    pub const fn as_char(self) -> char {
        RANK_CHARS[self as usize]
    }

    /// Returns the prime number associated with this rank.
    ///
    /// The prime numbers are the canonical values used by the Cactus Kev encoding for hand evaluation.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::Rank;
    /// assert_eq!(Rank::Ace.prime(), 41);
    /// ```
    #[must_use] 
    pub const fn prime(self) -> u32 {
        PRIMES[self as usize]
    }

    /// Compute the bit mask for this rank as used in card bitfield representations.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::Rank;
    /// let mask = Rank::Ace.bit_mask();
    /// assert_eq!(mask, 1u32 << (12 + 16));
    /// ```
    #[must_use] 
    pub const fn bit_mask(self) -> u32 {
        1u32 << (self as u8 + 16)
    }

    /// Iterate over all ranks from Two to Ace in ascending order.
    ///
    /// # Panics
    ///
    /// This function does not panic. The internal `unwrap()` is safe because all
    /// indices in the range `0..13` correspond to valid `Rank` variants.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::Rank;
    /// let ranks: Vec<_> = Rank::all().collect();
    /// assert_eq!(ranks.len(), 13);
    /// assert_eq!(ranks[0], Rank::Two);
    /// assert_eq!(ranks[12], Rank::Ace);
    /// ```
    pub fn all() -> impl Iterator<Item = Self> {
        (0..13).map(|i| Self::from_u8(i).unwrap())
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl FromStr for Rank {
    type Err = ParseCardError;

    /// Parses a single-character rank token into a `Rank`.
    ///
    /// Accepts characters `2`-`9`, `T`/`t`, `J`/`j`, `Q`/`q`, `K`/`k`, and `A`/`a`.
    ///
    /// # Returns
    ///
    /// `Ok(Rank)` for a valid single-character rank, `Err(ParseCardError::InvalidRank)` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use riverrun::core::domain::entities::card::Rank;
    /// assert_eq!(Rank::from_str("A").unwrap(), Rank::Ace);
    /// assert_eq!(Rank::from_str("t").unwrap(), Rank::Ten);
    /// assert!(Rank::from_str("10").is_err());
    /// assert!(Rank::from_str("x").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(ParseCardError::InvalidRank);
        }
        match s.chars().next().unwrap() {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' | 't' => Ok(Self::Ten),
            'J' | 'j' => Ok(Self::Jack),
            'Q' | 'q' => Ok(Self::Queen),
            'K' | 'k' => Ok(Self::King),
            'A' | 'a' => Ok(Self::Ace),
            _ => Err(ParseCardError::InvalidRank),
        }
    }
}

/// Error type for parsing cards.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseCardError {
    InvalidLength,
    InvalidRank,
    InvalidSuit,
}

impl fmt::Display for ParseCardError {
    /// Formats the parse error as a concise, human-readable message.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::ParseCardError;
    /// let e = ParseCardError::InvalidLength;
    /// assert_eq!(format!("{}", e), "card string must be exactly 2 characters");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "card string must be exactly 2 characters"),
            Self::InvalidRank => write!(f, "invalid rank character"),
            Self::InvalidSuit => write!(f, "invalid suit character"),
        }
    }
}

impl std::error::Error for ParseCardError {}

/// A Card entity.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Card(pub(crate) u32);

/// Card - Constructors
impl Card {
    /// Constructs a Card representing the specified rank and suit.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::{Card, Rank, Suit};
    /// let c = Card::new(Rank::Ace, Suit::Spades);
    /// assert_eq!(c.to_string(), "As");
    /// ```
    #[must_use]
    pub const fn new(rank: Rank, suit: Suit) -> Self {
        let prime = PRIMES[rank as usize];
        let rank_nibble = (rank as u32) << 8;
        let suit_bit = 1u32 << (suit as u8 + 12);
        let rank_bit = 1u32 << (rank as u8 + 16);

        Self(prime | rank_nibble | suit_bit | rank_bit)
    }

    /// Construct a Card from raw numeric rank (0 = Two, …, 12 = Ace) and suit (0 = Clubs, …, 3 = Spades).
    ///
    /// # Panics
    ///
    /// Panics if `rank` is not between 0 and 12 inclusive, or if `suit` is not between 0 and 3 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::Card;
    /// let c = Card::from_raw(12, 3); // Ace of Spades
    /// assert_eq!(c.index(), 51);
    /// ```
    #[must_use]
    pub const fn from_raw(rank: u8, suit: u8) -> Self {
        let Some(rank) = Rank::from_u8(rank) else {
            panic!("Rank must be 0-12")
        };
        let Some(suit) = Suit::from_u8(suit) else {
            panic!("Suit must be 0-3")
        };
        Self::new(rank, suit)
    }

    /// Creates a `Card` value for the given rank and suit.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::{Card, Rank, Suit};
    /// let c = Card::from_rank_suit(Rank::Ace, Suit::Spades);
    /// assert_eq!(c.to_string(), "As");
    /// ```
    #[must_use]
    pub const fn from_rank_suit(rank: Rank, suit: Suit) -> Self {
        Self::new(rank, suit)
    }

    /// Parse a playing card from a 2-character string like "As" or "Td".
    ///
    /// Returns `Some(Card)` when parsing succeeds, or `None` for invalid input.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::Card;
    /// let c = Card::from_string("As").unwrap();
    /// assert_eq!(c.to_string(), "As");
    /// ```
    #[must_use] 
    pub fn from_string(s: &str) -> Option<Self> {
        s.parse().ok()
    }

    /// Create the card corresponding to a 0-based index in the standard 52-card deck.
    ///
    /// The index maps ranks then suits (rank * 4 + suit); valid indices are 0 through 51.
    /// Returns `None` if the index is outside that range.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::Card;
    /// // 0 => 2 of clubs ("2c"), 51 => Ace of spades ("As")
    /// assert_eq!(Card::from_index(0).map(|c| c.to_string()), Some(String::from("2c")));
    /// assert_eq!(Card::from_index(51).map(|c| c.to_string()), Some(String::from("As")));
    /// assert!(Card::from_index(52).is_none());
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn from_index(index: usize) -> Option<Self> {
        if index >= 52 {
            return None;
        }
        let rank = Rank::from_u8((index / 4) as u8)?;
        let suit = Suit::from_u8((index % 4) as u8)?;
        Some(Self::new(rank, suit))
    }

    /// Generates the 52 cards of a standard deck.
    ///
    /// # Panics
    ///
    /// This function does not panic. The internal `unwrap()` is safe because all
    /// indices in the range `0..52` correspond to valid cards.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::Card;
    /// let cards: Vec<_> = Card::all_cards().collect();
    /// assert_eq!(cards.len(), 52);
    /// assert_eq!(cards[0].index(), 0);
    /// assert_eq!(cards[51].index(), 51);
    /// ```
    pub fn all_cards() -> impl Iterator<Item = Self> {
        (0..52).map(|i| Self::from_index(i).unwrap())
    }
}

/// Card - Accessors
impl Card {
    /// Accesses the card's underlying 32-bit Cactus Kev encoding.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::{Card, Rank, Suit};
    /// let card = Card::from_rank_suit(Rank::Two, Suit::Clubs);
    /// let raw: u32 = card.raw();
    /// let _ = raw; // raw is the packed 32-bit card encoding
    /// ``` 
    #[must_use] 
    pub const fn raw(&self) -> u32 {
        self.0
    }

    /// Extracts the prime-number component from a card's encoded representation.
    ///
    /// # Returns
    ///
    /// `u32` prime number associated with the card's rank (stored in bits 0–7).
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::{Card, Rank, Suit};
    /// let c = Card::new(Rank::Ace, Suit::Spades);
    /// assert_eq!(c.prime(), 41);
    /// ```
    #[inline]
    #[must_use]
    pub const fn prime(&self) -> u32 {
        self.0 & 0xFF
    }

    /// The card's rank as an index from 0 (Two) to 12 (Ace).
    ///
    /// # Returns
    ///
    /// `u8` in 0..=12 where 0 represents Two and 12 represents Ace.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::{Card, Rank, Suit};
    /// let c = Card::new(Rank::Two, Suit::Clubs);
    /// assert_eq!(c.rank(), 0);
    ///
    /// let a = Card::new(Rank::Ace, Suit::Spades);
    /// assert_eq!(a.rank(), 12);
    /// ```
    #[inline]
    #[must_use] 
    pub const fn rank(&self) -> u8 {
        ((self.0 >> 8) & 0xF) as u8
    }

    /// Return the card's rank as a `Rank` enum.
    ///
    /// # Panics
    ///
    /// Panics if the card's internal rank value is not between 0 and 12.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::{Card, Rank, Suit};
    /// let c = Card::new(Rank::Ace, Suit::Spades);
    /// assert_eq!(c.rank_enum(), Rank::Ace);
    /// ```
    #[inline]
    #[must_use]
    pub const fn rank_enum(&self) -> Rank {
        match Rank::from_u8(self.rank()) {
            Some(r) => r,
            None => panic!("Invalid rank"),
        }
    }

    /// Extracts the four-bit one-hot suit mask from the card encoding.
    ///
    /// The returned value is the four-bit mask stored in bits 12–15 of the
    /// card's internal representation, where each bit represents a suit.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::Card;
    /// let c = Card::from_string("As").unwrap(); // Ace of spades
    /// assert_eq!(c.suit_bits(), 0b1000);
    /// ```
    #[inline]
    #[must_use] 
    pub const fn suit_bits(&self) -> u32 {
        (self.0 >> 12) & 0xF
    }

    /// Extract the card's rank bitmask as a one-hot value.
    ///
    /// The returned `u32` contains a single bit set indicating the card's rank,
    /// where the bit corresponds to the rank index (0 = Two, 12 = Ace). This value
    /// is produced by taking the stored rank mask in bits 16–28 and shifting it
    /// down so the rank bit occupies the low 0–12 positions.
    ///
    /// # Returns
    ///
    /// `u32` with the rank's one-hot bit set (mapped from bits 16–28 into bit 0).
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::{Card, Rank, Suit};
    /// let c = Card::new(Rank::Ace, Suit::Spades);
    /// assert_eq!(c.rank_bits(), 1 << 12);
    /// ```
    #[inline]
    #[must_use]
    pub const fn rank_bits(&self) -> u32 {
        self.0 >> 16
    }

    /// The suit index of the card: 0 = Clubs, 1 = Diamonds, 2 = Hearts, 3 = Spades.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::Card;
    /// let two_clubs = Card::from_index(0).unwrap();
    /// assert_eq!(two_clubs.suit(), 0);
    ///
    /// let ace_spades = Card::from_index(51).unwrap();
    /// assert_eq!(ace_spades.suit(), 3);
    /// ```
    #[inline]
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn suit(&self) -> u8 {
        self.suit_bits().trailing_zeros() as u8
    }

    /// Retrieve the card's suit as a `Suit` enum.
    ///
    /// # Panics
    ///
    /// Panics if the card's internal suit value is not in 0..=3.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::{Card, Rank, Suit};
    /// let c = Card::from_rank_suit(Rank::Ace, Suit::Spades);
    /// assert_eq!(c.suit_enum(), Suit::Spades);
    /// ```
    #[inline]
    #[must_use]
    pub const fn suit_enum(&self) -> Suit {
        match Suit::from_u8(self.suit()) {
            Some(s) => s,
            None => panic!("Invalid suit"),
        }
    }

    /// Compute the card's unique index within a standard 52-card deck.
    ///
    /// The index is a value from 0 to 51 that uniquely identifies the card.
    ///
    /// # Returns
    ///
    /// The card's index in the range 0..=51.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::{Card, Rank, Suit};
    /// let two_clubs = Card::from_rank_suit(Rank::Two, Suit::Clubs);
    /// assert_eq!(two_clubs.index(), 0);
    ///
    /// let ace_spades = Card::from_rank_suit(Rank::Ace, Suit::Spades);
    /// assert_eq!(ace_spades.index(), 51);
    /// ```
    #[inline]
    #[must_use]
    pub const fn index(&self) -> usize {
        (self.rank() as usize) * 4 + (self.suit() as usize)
    }
}

/// Card - Operations
impl Card {
    /// Determines whether this card has the same rank as another.
    ///
    /// # Returns
    /// `true` if both cards have the same rank, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::Card;
    /// let a = Card::from_string("As").unwrap();
    /// let b = Card::from_string("Ah").unwrap();
    /// assert!(a.same_rank(&b));
    /// ```
    #[must_use] 
    pub const fn same_rank(&self, other: &Self) -> bool {
        self.rank() == other.rank()
    }

    /// Determines whether this card and another have the same suit.
    ///
    /// Returns `true` if they have the same suit, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::Card;
    /// let a = Card::from_string("As").unwrap();
    /// let b = Card::from_string("Ks").unwrap();
    /// assert!(a.same_suit(&b));
    /// ```
    #[inline]
    #[must_use]
    pub const fn same_suit(&self, other: &Self) -> bool {
        self.suit_bits() == other.suit_bits()
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank_enum(), self.suit_enum())
    }
}

impl FromStr for Card {
    type Err = ParseCardError;

    /// Parses a two-character card code (rank followed by suit) into a `Card`, e.g. `"As"`, `"Td"`, or `"2c"`.
    ///
    /// Returns `Ok(Card)` when the input is a valid two-character representation, or a `ParseCardError`
    /// indicating invalid length, rank, or suit on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::Card;
    /// let card = "As".parse::<Card>().unwrap();
    /// assert_eq!(card.to_string(), "As");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(ParseCardError::InvalidLength);
        }

        let mut chars = s.chars();
        let rank_char = chars.next().unwrap();
        let suit_char = chars.next().unwrap();

        let rank: Rank = rank_char.to_string().parse()?;
        let suit: Suit = suit_char.to_string().parse()?;

        Ok(Self::new(rank, suit))
    }
}

impl From<(Rank, Suit)> for Card {
    /// Converts a `(Rank, Suit)` pair into a `Card`.
    ///
    /// # Examples
    ///
    /// ```
    /// use riverrun::core::domain::entities::card::{Card, Rank, Suit};
    /// let card: Card = (Rank::Ace, Suit::Spades).into();
    /// assert_eq!(card.to_string(), "As");
    /// ```
    fn from((rank, suit): (Rank, Suit)) -> Self {
        Self::from_rank_suit(rank, suit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let card = Card::new(Rank::Ace, Suit::Spades);
        assert_eq!(card.rank(), 12);
        assert_eq!(card.suit(), 3);
        assert_eq!(card.prime(), 41);
    }

    #[test]
    fn test_card_from_raw() {
        let card = Card::from_raw(12, 3);
        assert_eq!(card.rank_enum(), Rank::Ace);
        assert_eq!(card.suit_enum(), Suit::Spades);
    }

    #[test]
    fn test_card_display() {
        assert_eq!(Card::new(Rank::Two, Suit::Clubs).to_string(), "2c");
        assert_eq!(Card::new(Rank::Ace, Suit::Spades).to_string(), "As");
        assert_eq!(Card::new(Rank::Ten, Suit::Diamonds).to_string(), "Td");
    }

    #[test]
    fn test_card_parse() {
        assert_eq!("As".parse::<Card>().unwrap(), Card::new(Rank::Ace, Suit::Spades));
        assert_eq!("2c".parse::<Card>().unwrap(), Card::new(Rank::Two, Suit::Clubs));
        assert_eq!("Td".parse::<Card>().unwrap(), Card::new(Rank::Ten, Suit::Diamonds));
        assert_eq!("qH".parse::<Card>().unwrap(), Card::new(Rank::Queen, Suit::Hearts));
    }

    #[test]
    fn test_card_parse_errors() {
        assert_eq!("A".parse::<Card>(), Err(ParseCardError::InvalidLength));
        assert_eq!("Asx".parse::<Card>(), Err(ParseCardError::InvalidLength));
        assert_eq!("Xs".parse::<Card>(), Err(ParseCardError::InvalidRank));
        assert_eq!("Ax".parse::<Card>(), Err(ParseCardError::InvalidSuit));
    }

    #[test]
    fn test_card_index() {
        let card = Card::new(Rank::Two, Suit::Clubs);
        assert_eq!(card.index(), 0);

        let card = Card::new(Rank::Ace, Suit::Spades);
        assert_eq!(card.index(), 51);
    }

    #[test]
    fn test_card_from_index() {
        assert_eq!(Card::from_index(0), Some(Card::new(Rank::Two, Suit::Clubs)));
        assert_eq!(Card::from_index(51), Some(Card::new(Rank::Ace, Suit::Spades)));
        assert_eq!(Card::from_index(52), None);
    }

    #[test]
    fn test_all_cards() {
        let cards: Vec<Card> = Card::all_cards().collect();
        assert_eq!(cards.len(), 52);
        assert_eq!(cards[0], Card::new(Rank::Two, Suit::Clubs));
        assert_eq!(cards[51], Card::new(Rank::Ace, Suit::Spades));
    }

    #[test]
    fn test_rank_bits() {
        let ace = Card::new(Rank::Ace, Suit::Clubs);
        let king = Card::new(Rank::King, Suit::Diamonds);
        assert_eq!(ace.rank_bits(), 1 << 12);
        assert_eq!(king.rank_bits(), 1 << 11);
    }

    #[test]
    fn test_suit_bits() {
        let clubs = Card::new(Rank::Two, Suit::Clubs);
        let spades = Card::new(Rank::Two, Suit::Spades);
        assert_eq!(clubs.suit_bits(), 0b0001);
        assert_eq!(spades.suit_bits(), 0b1000);
    }

    #[test]
    fn test_same_rank_suit() {
        let as_ = Card::new(Rank::Ace, Suit::Spades);
        let ah = Card::new(Rank::Ace, Suit::Hearts);
        let ks = Card::new(Rank::King, Suit::Spades);

        assert!(as_.same_rank(&ah));
        assert!(!as_.same_rank(&ks));
        assert!(as_.same_suit(&ks));
        assert!(!as_.same_suit(&ah));
    }

    #[test]
    fn test_suit_enum() {
        assert_eq!(Suit::from_u8(0), Some(Suit::Clubs));
        assert_eq!(Suit::from_u8(4), None);
        assert_eq!(Suit::Spades.as_char(), 's');
        assert_eq!("H".parse::<Suit>(), Ok(Suit::Hearts));
    }

    #[test]
    fn test_rank_enum() {
        assert_eq!(Rank::from_u8(12), Some(Rank::Ace));
        assert_eq!(Rank::from_u8(13), None);
        assert_eq!(Rank::Ace.prime(), 41);
        assert_eq!("K".parse::<Rank>(), Ok(Rank::King));
    }

    #[test]
    fn test_card_from_tuple() {
        let card: Card = (Rank::Ace, Suit::Spades).into();
        assert_eq!(card.to_string(), "As");
    }

    #[test]
    fn test_all_suits() {
        let suits: Vec<Suit> = Suit::all().collect();
        assert_eq!(suits.len(), 4);
    }

    #[test]
    fn test_all_ranks() {
        let ranks: Vec<Rank> = Rank::all().collect();
        assert_eq!(ranks.len(), 13);
        assert_eq!(ranks[0], Rank::Two);
        assert_eq!(ranks[12], Rank::Ace);
    }
}