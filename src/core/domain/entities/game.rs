//! Game representation for Poker (Texas Hold'em)

use super::board::{Board, Street};
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
    /// Create a new game with the specified number of players.
    /// Automatically creates and shuffles the deck.
    pub fn new<R: rand::Rng>(num_players: usize, rng: &mut R) -> Option<Self> {
        if num_players < 2 || num_players > 10 {
            return None;
        }

        let mut deck = Deck::new();
        deck.shuffle(rng);

        Some(Game {
            deck,
            num_players,
            hole_cards: Vec::new(),
            board: Board::new(),
        })
    }

    /// Create a new game with a pre-configured deck (for testing).
    pub fn with_deck(num_players: usize, deck: Deck) -> Option<Self> {
        if num_players < 2 || num_players > 10 {
            return None;
        }

        Some(Game {
            deck,
            num_players,
            hole_cards: Vec::new(),
            board: Board::new(),
        })
    }
}

/// Game - Accessors
impl Game {
    /// Get the number of players.
    pub fn num_players(&self) -> usize {
        self.num_players
    }

    /// Get the current street.
    pub fn street(&self) -> Street {
        self.board.street()
    }

    /// Get the board.
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Get hole cards for a specific player (0-indexed).
    pub fn player_hole_cards(&self, player: usize) -> Option<&[Card; 2]> {
        self.hole_cards.get(player)
    }

    /// Get all players' hole cards.
    pub fn all_hole_cards(&self) -> &[[Card; 2]] {
        &self.hole_cards
    }

    /// Get remaining cards in the deck.
    pub fn remaining_cards(&self) -> usize {
        self.deck.remaining()
    }

    /// Check if the game is at showdown (river dealt).
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
