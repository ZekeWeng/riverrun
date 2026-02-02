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
    /// Create a suit from a u8 value (0-3).
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Suit::Clubs),
            1 => Some(Suit::Diamonds),
            2 => Some(Suit::Hearts),
            3 => Some(Suit::Spades),
            _ => None,
        }
    }
}

/// Suit - Accessors
impl Suit {
    /// Get the character representation.
    pub fn as_char(self) -> char {
        SUIT_CHARS[self as usize]
    }

    /// Get the suit bit mask.
    pub fn bit_mask(self) -> u32 {
        1u32 << (self as u8 + 12)
    }

    /// Iterate over all suits.
    pub fn all() -> impl Iterator<Item = Suit> {
        [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades].into_iter()
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl FromStr for Suit {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(ParseCardError::InvalidSuit);
        }
        match s.chars().next().unwrap() {
            'c' | 'C' => Ok(Suit::Clubs),
            'd' | 'D' => Ok(Suit::Diamonds),
            'h' | 'H' => Ok(Suit::Hearts),
            's' | 'S' => Ok(Suit::Spades),
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
    /// Create a rank from a u8 value (0-12).
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Rank::Two),
            1 => Some(Rank::Three),
            2 => Some(Rank::Four),
            3 => Some(Rank::Five),
            4 => Some(Rank::Six),
            5 => Some(Rank::Seven),
            6 => Some(Rank::Eight),
            7 => Some(Rank::Nine),
            8 => Some(Rank::Ten),
            9 => Some(Rank::Jack),
            10 => Some(Rank::Queen),
            11 => Some(Rank::King),
            12 => Some(Rank::Ace),
            _ => None,
        }
    }
}

/// Rank - Accessors
impl Rank {
    /// Get the character representation.
    pub fn as_char(self) -> char {
        RANK_CHARS[self as usize]
    }

    /// Get the prime number for this rank.
    pub fn prime(self) -> u32 {
        PRIMES[self as usize]
    }

    /// Get the rank bit mask (for flush detection).
    pub fn bit_mask(self) -> u32 {
        1u32 << (self as u8 + 16)
    }

    /// Iterate over all ranks (2 to A).
    pub fn all() -> impl Iterator<Item = Rank> {
        (0..13).map(|i| Rank::from_u8(i).unwrap())
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl FromStr for Rank {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(ParseCardError::InvalidRank);
        }
        match s.chars().next().unwrap() {
            '2' => Ok(Rank::Two),
            '3' => Ok(Rank::Three),
            '4' => Ok(Rank::Four),
            '5' => Ok(Rank::Five),
            '6' => Ok(Rank::Six),
            '7' => Ok(Rank::Seven),
            '8' => Ok(Rank::Eight),
            '9' => Ok(Rank::Nine),
            'T' | 't' => Ok(Rank::Ten),
            'J' | 'j' => Ok(Rank::Jack),
            'Q' | 'q' => Ok(Rank::Queen),
            'K' | 'k' => Ok(Rank::King),
            'A' | 'a' => Ok(Rank::Ace),
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseCardError::InvalidLength => write!(f, "card string must be exactly 2 characters"),
            ParseCardError::InvalidRank => write!(f, "invalid rank character"),
            ParseCardError::InvalidSuit => write!(f, "invalid suit character"),
        }
    }
}

impl std::error::Error for ParseCardError {}

/// A Card entity.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Card(pub(crate) u32);

/// Card - Constructors
impl Card {
    /// Create a new card from Rank and Suit enums.
    pub fn new(rank: Rank, suit: Suit) -> Self {
        let prime = rank.prime();
        let rank_nibble = (rank as u32) << 8;
        let suit_bit = suit.bit_mask();
        let rank_bit = rank.bit_mask();

        Card(prime | rank_nibble | suit_bit | rank_bit)
    }

    /// Create a new card from raw rank and suit values.
    pub fn from_raw(rank: u8, suit: u8) -> Self {
        let rank = Rank::from_u8(rank).expect("Rank must be 0-12");
        let suit = Suit::from_u8(suit).expect("Suit must be 0-3");
        Self::new(rank, suit)
    }

    /// Alias for new().
    pub fn from_rank_suit(rank: Rank, suit: Suit) -> Self {
        Self::new(rank, suit)
    }

    /// Parse a card from a 2-character string (e.g., "As", "Td", "2c").
    pub fn from_string(s: &str) -> Option<Self> {
        s.parse().ok()
    }

    /// Create a card from its index (0-51).
    pub fn from_index(index: usize) -> Option<Self> {
        if index >= 52 {
            return None;
        }
        let rank = Rank::from_u8((index / 4) as u8)?;
        let suit = Suit::from_u8((index % 4) as u8)?;
        Some(Self::new(rank, suit))
    }

    /// Generate all 52 cards in a standard deck.
    pub fn all_cards() -> impl Iterator<Item = Card> {
        (0..52).map(|i| Self::from_index(i).unwrap())
    }
}

/// Card - Accessors
impl Card {
    /// Get the raw 32-bit encoding.
    #[inline]
    pub fn raw(&self) -> u32 {
        self.0
    }

    /// Get the prime number component.
    #[inline]
    pub fn prime(&self) -> u32 {
        self.0 & 0xFF
    }

    /// Get the rank as a u8 (0-12).
    #[inline]
    pub fn rank(&self) -> u8 {
        ((self.0 >> 8) & 0xF) as u8
    }

    /// Get the rank as an enum.
    #[inline]
    pub fn rank_enum(&self) -> Rank {
        Rank::from_u8(self.rank()).unwrap()
    }

    /// Get the suit bits (one-hot encoded, bits 12-15).
    #[inline]
    pub fn suit_bits(&self) -> u32 {
        (self.0 >> 12) & 0xF
    }

    /// Get the rank bits (one-hot encoded, bits 16-28).
    #[inline]
    pub fn rank_bits(&self) -> u32 {
        self.0 >> 16
    }

    /// Get the suit as a u8 (0-3).
    #[inline]
    pub fn suit(&self) -> u8 {
        self.suit_bits().trailing_zeros() as u8
    }

    /// Get the suit as an enum.
    #[inline]
    pub fn suit_enum(&self) -> Suit {
        Suit::from_u8(self.suit()).unwrap()
    }

    /// Get the unique index of this card (0-51).
    #[inline]
    pub fn index(&self) -> usize {
        (self.rank() as usize) * 4 + (self.suit() as usize)
    }
}

/// Card - Operations
impl Card {
    /// Check if this card has the same rank as another.
    #[inline]
    pub fn same_rank(&self, other: &Card) -> bool {
        self.rank() == other.rank()
    }

    /// Check if this card has the same suit as another.
    #[inline]
    pub fn same_suit(&self, other: &Card) -> bool {
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(ParseCardError::InvalidLength);
        }

        let mut chars = s.chars();
        let rank_char = chars.next().unwrap();
        let suit_char = chars.next().unwrap();

        let rank: Rank = rank_char.to_string().parse()?;
        let suit: Suit = suit_char.to_string().parse()?;

        Ok(Card::new(rank, suit))
    }
}

impl From<(Rank, Suit)> for Card {
    fn from((rank, suit): (Rank, Suit)) -> Self {
        Card::from_rank_suit(rank, suit)
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
