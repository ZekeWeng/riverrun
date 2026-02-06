//! Betting and pot state primitives.

use super::{Chips, PlayerId};
use std::fmt;

/// Represents a pot in a poker hand.
///
/// Supports side pots for all-in situations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pot {
    /// Total chips in this pot.
    amount: Chips,
    /// Players eligible to win this pot.
    eligible_players: Vec<PlayerId>,
}

impl Pot {
    /// Creates a new empty pot.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            amount: Chips::ZERO,
            eligible_players: Vec::new(),
        }
    }

    /// Creates a pot with initial amount and eligible players.
    #[must_use]
    pub const fn with_players(amount: Chips, eligible_players: Vec<PlayerId>) -> Self {
        Self {
            amount,
            eligible_players,
        }
    }

    /// Returns the total chips in this pot.
    #[must_use]
    pub const fn amount(&self) -> Chips {
        self.amount
    }

    /// Returns the players eligible to win this pot.
    #[must_use]
    pub fn eligible_players(&self) -> &[PlayerId] {
        &self.eligible_players
    }

    /// Adds chips to the pot.
    pub fn add(&mut self, chips: Chips) {
        self.amount += chips;
    }

    /// Adds a player to the eligible list.
    pub fn add_eligible_player(&mut self, player: PlayerId) {
        if !self.eligible_players.contains(&player) {
            self.eligible_players.push(player);
        }
    }

    /// Removes a player from eligibility (e.g., when they fold).
    pub fn remove_eligible_player(&mut self, player: PlayerId) {
        self.eligible_players.retain(|p| *p != player);
    }

    /// Returns whether a player is eligible to win this pot.
    #[must_use]
    pub fn is_eligible(&self, player: PlayerId) -> bool {
        self.eligible_players.contains(&player)
    }

    /// Returns the number of eligible players.
    #[must_use]
    pub fn eligible_count(&self) -> usize {
        self.eligible_players.len()
    }

    /// Resets the pot to empty state.
    pub fn clear(&mut self) {
        self.amount = Chips::ZERO;
        self.eligible_players.clear();
    }
}

impl Default for Pot {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Pot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pot: {} ({} eligible)", self.amount, self.eligible_players.len())
    }
}

/// Tracks betting state for a single street.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BettingRound {
    /// Current bet amount to call.
    current_bet: Chips,
    /// Minimum raise amount.
    min_raise: Chips,
    /// Amount each player has contributed this round.
    contributions: Vec<Chips>,
    /// Whether each player has acted this round.
    has_acted: Vec<bool>,
    /// Number of raises made this round.
    raise_count: u8,
}

impl BettingRound {
    /// Creates a new betting round for the given number of players.
    #[must_use]
    pub fn new(num_players: usize, big_blind: Chips) -> Self {
        Self {
            current_bet: big_blind,
            min_raise: big_blind,
            contributions: vec![Chips::ZERO; num_players],
            has_acted: vec![false; num_players],
            raise_count: 0,
        }
    }

    /// Creates a betting round for post-flop streets.
    #[must_use]
    pub fn new_postflop(num_players: usize, big_blind: Chips) -> Self {
        Self {
            current_bet: Chips::ZERO,
            min_raise: big_blind,
            contributions: vec![Chips::ZERO; num_players],
            has_acted: vec![false; num_players],
            raise_count: 0,
        }
    }

    /// Returns the current bet amount to call.
    #[must_use]
    pub const fn current_bet(&self) -> Chips {
        self.current_bet
    }

    /// Returns the minimum raise amount.
    #[must_use]
    pub const fn min_raise(&self) -> Chips {
        self.min_raise
    }

    /// Returns how much a player has contributed this round.
    #[must_use]
    pub fn player_contribution(&self, player: PlayerId) -> Chips {
        self.contributions
            .get(player.as_index())
            .copied()
            .unwrap_or(Chips::ZERO)
    }

    /// Returns how much more a player needs to call.
    #[must_use]
    pub fn amount_to_call(&self, player: PlayerId) -> Chips {
        self.current_bet.saturating_sub(self.player_contribution(player))
    }

    /// Returns whether a player has acted this round.
    #[must_use]
    pub fn has_acted(&self, player: PlayerId) -> bool {
        self.has_acted
            .get(player.as_index())
            .copied()
            .unwrap_or(false)
    }

    /// Returns the number of raises this round.
    #[must_use]
    pub const fn raise_count(&self) -> u8 {
        self.raise_count
    }

    /// Records a bet/raise from a player.
    ///
    /// # Arguments
    /// * `player` - The player making the bet
    /// * `total_bet` - The total amount the player has bet (not additional)
    ///
    /// # Returns
    /// The additional chips put in by the player.
    pub fn record_bet(&mut self, player: PlayerId, total_bet: Chips) -> Chips {
        let idx = player.as_index();
        let previous = self.contributions.get(idx).copied().unwrap_or(Chips::ZERO);
        let additional = total_bet.saturating_sub(previous);

        if idx < self.contributions.len() {
            self.contributions[idx] = total_bet;
        }
        if idx < self.has_acted.len() {
            self.has_acted[idx] = true;
        }

        // Update current bet and min raise if this is a raise
        if total_bet > self.current_bet {
            let raise_amount = total_bet - self.current_bet;
            self.min_raise = self.min_raise.max(raise_amount);
            self.current_bet = total_bet;
            self.raise_count += 1;
            // Reset has_acted for other players who need to respond
            for (i, acted) in self.has_acted.iter_mut().enumerate() {
                if i != idx {
                    *acted = false;
                }
            }
        }

        additional
    }

    /// Records a check action.
    pub fn record_check(&mut self, player: PlayerId) {
        let idx = player.as_index();
        if idx < self.has_acted.len() {
            self.has_acted[idx] = true;
        }
    }

    /// Records a fold action.
    pub fn record_fold(&mut self, player: PlayerId) {
        let idx = player.as_index();
        if idx < self.has_acted.len() {
            self.has_acted[idx] = true;
        }
    }

    /// Returns the minimum legal raise-to amount.
    #[must_use]
    pub fn min_raise_to(&self) -> Chips {
        self.current_bet + self.min_raise
    }

    /// Returns total contributions across all players this round.
    #[must_use]
    pub fn total_contributions(&self) -> Chips {
        self.contributions.iter().fold(Chips::ZERO, |acc, c| acc + *c)
    }
}

/// Complete betting state for a hand.
#[derive(Clone, Debug)]
pub struct BettingState {
    /// Main pot.
    main_pot: Pot,
    /// Side pots (for all-in situations).
    side_pots: Vec<Pot>,
    /// Current betting round.
    current_round: Option<BettingRound>,
    /// Total chips invested by each player across all rounds.
    total_invested: Vec<Chips>,
    /// Stack size for each player.
    stacks: Vec<Chips>,
    /// Whether each player is all-in.
    is_all_in: Vec<bool>,
    /// Whether each player has folded.
    has_folded: Vec<bool>,
}

impl BettingState {
    /// Creates a new betting state.
    #[must_use]
    pub fn new(stacks: Vec<Chips>) -> Self {
        let num_players = stacks.len();
        Self {
            main_pot: Pot::new(),
            side_pots: Vec::new(),
            current_round: None,
            total_invested: vec![Chips::ZERO; num_players],
            stacks,
            is_all_in: vec![false; num_players],
            has_folded: vec![false; num_players],
        }
    }

    /// Returns the number of players.
    #[must_use]
    pub fn num_players(&self) -> usize {
        self.stacks.len()
    }

    /// Returns the main pot.
    #[must_use]
    pub const fn main_pot(&self) -> &Pot {
        &self.main_pot
    }

    /// Returns the side pots.
    #[must_use]
    pub fn side_pots(&self) -> &[Pot] {
        &self.side_pots
    }

    /// Returns total pot size (main + side pots).
    #[must_use]
    pub fn total_pot(&self) -> Chips {
        let side = self.side_pots.iter().fold(Chips::ZERO, |acc, p| acc + p.amount());
        self.main_pot.amount() + side
    }

    /// Returns a player's current stack.
    #[must_use]
    pub fn stack(&self, player: PlayerId) -> Chips {
        self.stacks
            .get(player.as_index())
            .copied()
            .unwrap_or(Chips::ZERO)
    }

    /// Returns whether a player is all-in.
    #[must_use]
    pub fn is_all_in(&self, player: PlayerId) -> bool {
        self.is_all_in
            .get(player.as_index())
            .copied()
            .unwrap_or(false)
    }

    /// Returns whether a player has folded.
    #[must_use]
    pub fn has_folded(&self, player: PlayerId) -> bool {
        self.has_folded
            .get(player.as_index())
            .copied()
            .unwrap_or(true)
    }

    /// Returns whether a player is still active (not folded, not all-in).
    #[must_use]
    pub fn is_active(&self, player: PlayerId) -> bool {
        !self.has_folded(player) && !self.is_all_in(player)
    }

    /// Returns the number of active players.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn active_count(&self) -> usize {
        (0..self.num_players())
            .filter(|&i| self.is_active(PlayerId::new(i as u8)))
            .count()
    }

    /// Returns the number of players still in the hand (not folded).
    #[must_use]
    pub fn players_in_hand(&self) -> usize {
        self.has_folded.iter().filter(|&&f| !f).count()
    }

    /// Returns the current betting round if one is active.
    #[must_use]
    pub const fn current_round(&self) -> Option<&BettingRound> {
        self.current_round.as_ref()
    }

    /// Starts a new betting round.
    pub fn start_round(&mut self, big_blind: Chips, is_preflop: bool) {
        let num_players = self.num_players();
        self.current_round = Some(if is_preflop {
            BettingRound::new(num_players, big_blind)
        } else {
            BettingRound::new_postflop(num_players, big_blind)
        });
    }

    /// Posts a blind.
    pub fn post_blind(&mut self, player: PlayerId, amount: Chips) {
        let idx = player.as_index();
        let actual = amount.min(self.stacks[idx]);

        self.stacks[idx] = self.stacks[idx].saturating_sub(actual);
        self.total_invested[idx] += actual;
        self.main_pot.add(actual);
        self.main_pot.add_eligible_player(player);

        if self.stacks[idx].is_zero() {
            self.is_all_in[idx] = true;
        }

        if let Some(round) = &mut self.current_round {
            round.record_bet(player, actual);
        }
    }

    /// Records a fold.
    pub fn fold(&mut self, player: PlayerId) {
        let idx = player.as_index();
        if idx < self.has_folded.len() {
            self.has_folded[idx] = true;
        }
        self.main_pot.remove_eligible_player(player);
        for pot in &mut self.side_pots {
            pot.remove_eligible_player(player);
        }
        if let Some(round) = &mut self.current_round {
            round.record_fold(player);
        }
    }

    /// Records a check.
    pub fn check(&mut self, player: PlayerId) {
        if let Some(round) = &mut self.current_round {
            round.record_check(player);
        }
    }

    /// Records a call.
    ///
    /// # Returns
    /// The actual amount called (may be less if player goes all-in).
    pub fn call(&mut self, player: PlayerId) -> Chips {
        let idx = player.as_index();
        let to_call = self
            .current_round
            .as_ref()
            .map_or(Chips::ZERO, |r| r.amount_to_call(player));
        let actual = to_call.min(self.stacks[idx]);

        self.stacks[idx] = self.stacks[idx].saturating_sub(actual);
        self.total_invested[idx] += actual;
        self.main_pot.add(actual);

        if self.stacks[idx].is_zero() {
            self.is_all_in[idx] = true;
        }

        if let Some(round) = &mut self.current_round {
            let current_contribution = round.player_contribution(player);
            round.record_bet(player, current_contribution + actual);
        }

        actual
    }

    /// Records a bet or raise.
    ///
    /// # Arguments
    /// * `player` - The player betting/raising
    /// * `total_bet` - The total bet amount (not additional)
    ///
    /// # Returns
    /// The actual additional amount put in.
    pub fn bet_or_raise(&mut self, player: PlayerId, total_bet: Chips) -> Chips {
        let idx = player.as_index();
        let current_contribution = self
            .current_round
            .as_ref()
            .map_or(Chips::ZERO, |r| r.player_contribution(player));
        let additional = total_bet.saturating_sub(current_contribution);
        let actual = additional.min(self.stacks[idx]);

        self.stacks[idx] = self.stacks[idx].saturating_sub(actual);
        self.total_invested[idx] += actual;
        self.main_pot.add(actual);

        if self.stacks[idx].is_zero() {
            self.is_all_in[idx] = true;
        }

        if let Some(round) = &mut self.current_round {
            round.record_bet(player, current_contribution + actual);
        }

        actual
    }

    /// Ends the current betting round.
    pub fn end_round(&mut self) {
        self.current_round = None;
    }

    /// Returns total chips invested by a player.
    #[must_use]
    pub fn total_invested(&self, player: PlayerId) -> Chips {
        self.total_invested
            .get(player.as_index())
            .copied()
            .unwrap_or(Chips::ZERO)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pot_basic() {
        let mut pot = Pot::new();
        assert!(pot.amount().is_zero());

        pot.add(Chips::new(50));
        pot.add(Chips::new(50));
        assert_eq!(pot.amount(), Chips::new(100));
    }

    #[test]
    fn test_pot_eligibility() {
        let mut pot = Pot::new();
        let p0 = PlayerId::new(0);
        let p1 = PlayerId::new(1);

        pot.add_eligible_player(p0);
        pot.add_eligible_player(p1);
        assert!(pot.is_eligible(p0));
        assert!(pot.is_eligible(p1));
        assert_eq!(pot.eligible_count(), 2);

        pot.remove_eligible_player(p0);
        assert!(!pot.is_eligible(p0));
        assert_eq!(pot.eligible_count(), 1);
    }

    #[test]
    fn test_betting_round() {
        let mut round = BettingRound::new(6, Chips::new(10));
        let p0 = PlayerId::new(0);

        assert_eq!(round.current_bet(), Chips::new(10));
        assert_eq!(round.amount_to_call(p0), Chips::new(10));

        round.record_bet(p0, Chips::new(10));
        assert_eq!(round.player_contribution(p0), Chips::new(10));
        assert!(round.has_acted(p0));
    }

    #[test]
    fn test_betting_round_raise() {
        let mut round = BettingRound::new(3, Chips::new(10));
        let p0 = PlayerId::new(0);
        let p1 = PlayerId::new(1);

        round.record_bet(p0, Chips::new(10)); // call
        round.record_bet(p1, Chips::new(30)); // raise to 30

        assert_eq!(round.current_bet(), Chips::new(30));
        assert_eq!(round.raise_count(), 1);
        assert!(!round.has_acted(p0)); // p0 needs to act again
    }

    #[test]
    fn test_betting_state() {
        let stacks = vec![Chips::new(1000), Chips::new(1000), Chips::new(1000)];
        let state = BettingState::new(stacks);

        assert_eq!(state.num_players(), 3);
        assert_eq!(state.stack(PlayerId::new(0)), Chips::new(1000));
    }

    #[test]
    fn test_betting_state_blinds() {
        let stacks = vec![Chips::new(1000), Chips::new(1000), Chips::new(1000)];
        let mut state = BettingState::new(stacks);
        state.start_round(Chips::new(10), true);

        let sb = PlayerId::new(0);
        let bb = PlayerId::new(1);

        state.post_blind(sb, Chips::new(5));
        state.post_blind(bb, Chips::new(10));

        assert_eq!(state.stack(sb), Chips::new(995));
        assert_eq!(state.stack(bb), Chips::new(990));
        assert_eq!(state.total_pot(), Chips::new(15));
    }

    #[test]
    fn test_betting_state_fold() {
        let stacks = vec![Chips::new(1000), Chips::new(1000)];
        let mut state = BettingState::new(stacks);

        let p0 = PlayerId::new(0);
        state.fold(p0);

        assert!(state.has_folded(p0));
        assert!(!state.is_active(p0));
        assert_eq!(state.players_in_hand(), 1);
    }

    #[test]
    fn test_betting_state_all_in() {
        let stacks = vec![Chips::new(100), Chips::new(1000)];
        let mut state = BettingState::new(stacks);
        state.start_round(Chips::new(10), true);

        let p0 = PlayerId::new(0);
        state.bet_or_raise(p0, Chips::new(100));

        assert!(state.is_all_in(p0));
        assert_eq!(state.stack(p0), Chips::ZERO);
    }
}
