//! Game representation for Poker (Texas Hold'em)

use super::board::Board;
use crate::core::domain::primitives::Street;
use super::card::Card;
use super::deck::Deck;

/// A Texas Hold'em poker game.
#[derive(Clone, Debug)]
pub struct Game {
    deck: Deck,
    num_players: usize,
    hole_cards: Vec<[Card; 2]>,
    board: Board,
}

/// Game - Constructors
impl Game {
    /// Constructs a new `Game` for the given number of players with a shuffled deck.
    ///
    /// The `num_players` must be between 2 and 10 inclusive; returns `None` if the value is out of range.
    /// The returned `Game` has an initialized, shuffled deck, no dealt hole cards, and an empty board.
    pub fn new<R: rand::Rng>(num_players: usize, rng: &mut R) -> Option<Self> {
        if !(2..=10).contains(&num_players) {
            return None;
        }

        let mut deck = Deck::new();
        deck.shuffle(rng);

        Some(Self {
            deck,
            num_players,
            hole_cards: Vec::new(),
            board: Board::new(),
        })
    }

    /// Constructs a Game using the provided deck and player count.
    ///
    /// The `num_players` must be between 2 and 10 inclusive; the function returns `None` if the count is out of range.
    #[must_use]
    pub fn with_deck(num_players: usize, deck: Deck) -> Option<Self> {
        if !(2..=10).contains(&num_players) {
            return None;
        }

        Some(Self {
            deck,
            num_players,
            hole_cards: Vec::new(),
            board: Board::new(),
        })
    }
}

/// Game - Accessors
impl Game {
    /// Number of players configured for this game.
    #[must_use]
    pub const fn num_players(&self) -> usize {
        self.num_players
    }

    /// Get the current street.
    #[must_use]
    pub const fn street(&self) -> Street {
        self.board.street()
    }

    /// Access the game's current board.
    ///
    /// # Returns
    ///
    /// A reference to the game's current `Board`.
    #[must_use]
    pub const fn board(&self) -> &Board {
        &self.board
    }

    /// Retrieve the hole cards for the specified player.
    ///
    /// The `player` index is zero-based.
    #[must_use]
    pub fn player_hole_cards(&self, player: usize) -> Option<&[Card; 2]> {
        self.hole_cards.get(player)
    }

    /// Provides a slice of all players' hole cards in seating order.
    ///
    /// Each element is a two-card array representing a player's hole cards; the slice is empty before hole cards are dealt.
    #[must_use]
    pub fn all_hole_cards(&self) -> &[[Card; 2]] {
        &self.hole_cards
    }

    /// Reports the number of cards remaining in the game's deck.
    ///
    /// # Returns
    ///
    /// The count of undealt cards left in the deck.
    #[must_use]
    pub fn remaining_cards(&self) -> usize {
        self.deck.remaining()
    }

    /// Returns whether the game has reached showdown (the river has been dealt).
    #[must_use]
    pub fn is_showdown(&self) -> bool {
        self.board.street() == Street::River
    }
}

/// Game - Operations
impl Game {
    /// Deal hole cards to all players.
    /// Returns false if hole cards have already been dealt.
    pub fn deal_hole_cards(&mut self) -> bool {
        if !self.hole_cards.is_empty() {
            return false;
        }

        match self.deck.deal_hole_cards(self.num_players) {
            Some(cards) => {
                self.hole_cards = cards;
                true
            }
            None => false,
        }
    }

    /// Deal the flop.
    /// Returns false if not at the correct stage or already dealt.
    pub fn deal_flop(&mut self) -> bool {
        if self.board.street() != Street::Preflop {
            return false;
        }

        match self.deck.deal_flop() {
            Some([c1, c2, c3]) => self.board.deal_flop(c1, c2, c3),
            None => false,
        }
    }

    /// Deal the turn.
    /// Returns false if not at the correct stage.
    pub fn deal_turn(&mut self) -> bool {
        if self.board.street() != Street::Flop {
            return false;
        }

        match self.deck.deal_turn() {
            Some(card) => self.board.deal_turn(card),
            None => false,
        }
    }

    /// Deal the river.
    /// Returns false if not at the correct stage.
    pub fn deal_river(&mut self) -> bool {
        if self.board.street() != Street::Turn {
            return false;
        }

        match self.deck.deal_river() {
            Some(card) => self.board.deal_river(card),
            None => false,
        }
    }

    /// Deal all cards up to and including the river.
    /// Returns false if any deal fails.
    pub fn deal_to_river(&mut self) -> bool {
        if !self.deal_hole_cards() {
            return false;
        }
        if !self.deal_flop() {
            return false;
        }
        if !self.deal_turn() {
            return false;
        }
        self.deal_river()
    }

    /// Reset the game for a new hand.
    pub fn reset<R: rand::Rng>(&mut self, rng: &mut R) {
        self.deck.reset();
        self.deck.shuffle(rng);
        self.hole_cards.clear();
        self.board.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn make_rng() -> rand::rngs::StdRng {
        rand::rngs::StdRng::seed_from_u64(42)
    }

    #[test]
    fn test_new_game() {
        let mut rng = make_rng();
        let game = Game::new(6, &mut rng).unwrap();
        assert_eq!(game.num_players(), 6);
        assert_eq!(game.street(), Street::Preflop);
        assert!(game.all_hole_cards().is_empty());
    }

    #[test]
    fn test_new_game_invalid_players() {
        let mut rng = make_rng();
        assert!(Game::new(1, &mut rng).is_none()); // Too few
        assert!(Game::new(11, &mut rng).is_none()); // Too many
    }

    #[test]
    fn test_deal_hole_cards() {
        let mut rng = make_rng();
        let mut game = Game::new(4, &mut rng).unwrap();
        assert!(game.deal_hole_cards());
        assert_eq!(game.all_hole_cards().len(), 4);
        assert_eq!(game.remaining_cards(), 44);
    }

    #[test]
    fn test_deal_hole_cards_twice() {
        let mut rng = make_rng();
        let mut game = Game::new(4, &mut rng).unwrap();
        assert!(game.deal_hole_cards());
        assert!(!game.deal_hole_cards()); // Can't deal twice
    }

    #[test]
    fn test_deal_flop() {
        let mut rng = make_rng();
        let mut game = Game::new(4, &mut rng).unwrap();
        game.deal_hole_cards();
        assert!(game.deal_flop());
        assert_eq!(game.street(), Street::Flop);
        assert_eq!(game.board().len(), 3);
    }

    #[test]
    fn test_deal_flop_wrong_stage() {
        let mut rng = make_rng();
        let mut game = Game::new(4, &mut rng).unwrap();
        game.deal_hole_cards();
        game.deal_flop();
        assert!(!game.deal_flop()); // Can't deal flop twice
    }

    #[test]
    fn test_deal_turn() {
        let mut rng = make_rng();
        let mut game = Game::new(4, &mut rng).unwrap();
        game.deal_hole_cards();
        game.deal_flop();
        assert!(game.deal_turn());
        assert_eq!(game.street(), Street::Turn);
        assert_eq!(game.board().len(), 4);
    }

    #[test]
    fn test_deal_river() {
        let mut rng = make_rng();
        let mut game = Game::new(4, &mut rng).unwrap();
        game.deal_hole_cards();
        game.deal_flop();
        game.deal_turn();
        assert!(game.deal_river());
        assert_eq!(game.street(), Street::River);
        assert_eq!(game.board().len(), 5);
        assert!(game.is_showdown());
    }

    #[test]
    fn test_deal_to_river() {
        let mut rng = make_rng();
        let mut game = Game::new(6, &mut rng).unwrap();
        assert!(game.deal_to_river());
        assert!(game.is_showdown());
        assert_eq!(game.all_hole_cards().len(), 6);
        assert_eq!(game.board().len(), 5);
    }

    #[test]
    fn test_player_hole_cards() {
        let mut rng = make_rng();
        let mut game = Game::new(3, &mut rng).unwrap();
        game.deal_hole_cards();
        assert!(game.player_hole_cards(0).is_some());
        assert!(game.player_hole_cards(2).is_some());
        assert!(game.player_hole_cards(3).is_none()); // Out of bounds
    }

    #[test]
    fn test_reset() {
        let mut rng = make_rng();
        let mut game = Game::new(4, &mut rng).unwrap();
        game.deal_to_river();
        assert!(game.is_showdown());

        game.reset(&mut rng);
        assert_eq!(game.street(), Street::Preflop);
        assert!(game.all_hole_cards().is_empty());
        assert_eq!(game.remaining_cards(), 52);
    }

    #[test]
    fn test_remaining_cards_progression() {
        let mut rng = make_rng();
        let mut game = Game::new(2, &mut rng).unwrap();
        assert_eq!(game.remaining_cards(), 52);

        game.deal_hole_cards(); // 4 cards dealt
        assert_eq!(game.remaining_cards(), 48);

        game.deal_flop(); // 1 burn + 3 dealt
        assert_eq!(game.remaining_cards(), 44);

        game.deal_turn(); // 1 burn + 1 dealt
        assert_eq!(game.remaining_cards(), 42);

        game.deal_river(); // 1 burn + 1 dealt
        assert_eq!(game.remaining_cards(), 40);
    }

    #[test]
    fn test_with_deck() {
        let deck = Deck::new();
        let game = Game::with_deck(4, deck).unwrap();
        assert_eq!(game.num_players(), 4);
        assert_eq!(game.remaining_cards(), 52);
    }
}
