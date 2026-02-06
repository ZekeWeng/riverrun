//! Precomputed hand rank tables for Cactus Kev evaluation.
//!
//! Provides O(1) flush lookup and O(log n) non-flush lookup using
//! prime product hashing.

use std::collections::HashMap;

use super::super::utils::{combinations, is_straight_pattern};

/// Prime numbers mapped to card ranks (2-A).
pub const PRIMES: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

/// Worst possible hand rank (7-high).
pub const WORST_RANK: u16 = 7462;

/// Precomputed lookup tables for fast hand evaluation.
///
/// Two lookup structures:
/// - `flush_lookup`: 8192-entry table indexed by rank bits for flush hands
/// - `unique5`: Sorted (`prime_product`, rank) pairs for non-flush hands
pub struct HandRankTables {
    pub flush_lookup: Vec<u16>,
    pub unique5: Vec<(u32, u16)>,
}

/// `HandRankTables` - Constructors
impl HandRankTables {
    /// Constructs precomputed hand-rank lookup tables used by the Cactus Kev evaluator.
    ///
    /// The returned `HandRankTables` contains:
    /// - a `flush_lookup` table (8192 entries) for O(1) flush-hand rank lookup by rank-bit index,
    /// - a sorted `unique5` table of (prime-product, rank) pairs for non-flush hands (binary-searchable).
    ///
    /// The tables are populated once in descending hand strength order so that ranks reflect poker hand strength.
    #[must_use] 
    pub fn new() -> Self {
        let mut flush_lookup = vec![WORST_RANK; 8192];
        let mut unique5_map: HashMap<u32, u16> = HashMap::new();

        let mut current_rank = 1u16;

        // Generate in order of hand strength (best to worst)
        current_rank = generate_straight_flushes(&mut flush_lookup, current_rank);
        current_rank = generate_four_of_kind(&mut unique5_map, current_rank);
        current_rank = generate_full_houses(&mut unique5_map, current_rank);
        current_rank = generate_flushes(&mut flush_lookup, current_rank);
        current_rank = generate_straights(&mut unique5_map, current_rank);
        current_rank = generate_three_of_kind(&mut unique5_map, current_rank);
        current_rank = generate_two_pair(&mut unique5_map, current_rank);
        current_rank = generate_one_pair(&mut unique5_map, current_rank);
        generate_high_card(&mut unique5_map, current_rank);

        // Convert HashMap to sorted Vec for binary search
        let mut unique5: Vec<(u32, u16)> = unique5_map.into_iter().collect();
        unique5.sort_by_key(|(product, _)| *product);

        Self {
            flush_lookup,
            unique5,
        }
    }
}

impl Default for HandRankTables {
    /// Creates a `HandRankTables` populated with the precomputed hand rank lookup tables.
    fn default() -> Self {
        Self::new()
    }
}

/// `HandRankTables` - Accessors
impl HandRankTables {
    /// Retrieve the hand rank for a flush pattern identified by a rank-bit index.
    ///
    /// `rank_bits` is the rank-bit index into the flush lookup table (valid range: 0..8192).
    ///
    /// # Returns
    ///
    /// The hand rank corresponding to the flush pattern.
    #[must_use] 
    pub fn lookup_flush(&self, rank_bits: u32) -> u16 {
        self.flush_lookup[rank_bits as usize]
    }

    /// Lookup the hand rank for a 5-card non-flush hand identified by its prime-product key.
    ///
    /// The `prime_product` is the product of the per-rank primes for the five ranks in the hand.
    ///
    /// # Returns
    ///
    /// `Some(rank)` with the hand's rank if the product is found in the non-flush table, `None` otherwise.
    #[must_use] 
    pub fn lookup_unique(&self, prime_product: u32) -> Option<u16> {
        self.unique5
            .binary_search_by_key(&prime_product, |&(p, _)| p)
            .ok()
            .map(|idx| self.unique5[idx].1)
    }
}

// Straight flush bit patterns (A-high to wheel)
/// Populate the flush lookup table with straight-flush hand ranks in descending strength.
///
/// This assigns consecutive rank values for the ten straight-flush patterns (A-high down to the five-high wheel)
/// into `flush_lookup` at indices equal to each pattern's rank-bit mask.
///
/// # Returns
///
/// The next rank value after the last rank assigned.
fn generate_straight_flushes(flush_lookup: &mut [u16], mut rank: u16) -> u16 {
    let patterns = [
        0b1_1111_0000_0000_u32, // A-K-Q-J-T
        0b0_1111_1000_0000,     // K-Q-J-T-9
        0b0_0111_1100_0000,     // Q-J-T-9-8
        0b0_0011_1110_0000,     // J-T-9-8-7
        0b0_0001_1111_0000,     // T-9-8-7-6
        0b0_0000_1111_1000,     // 9-8-7-6-5
        0b0_0000_0111_1100,     // 8-7-6-5-4
        0b0_0000_0011_1110,     // 7-6-5-4-3
        0b0_0000_0001_1111,     // 6-5-4-3-2
        0b1_0000_0000_1111,     // 5-4-3-2-A
    ];

    for pattern in patterns {
        flush_lookup[pattern as usize] = rank;
        rank += 1;
    }
    rank
}

fn generate_four_of_kind(map: &mut HashMap<u32, u16>, mut rank: u16) -> u16 {
    for quad_rank in (0..13).rev() {
        let quad_prime = PRIMES[quad_rank];

        for kicker in (0..13).rev() {
            if kicker == quad_rank {
                continue;
            }

            let product = quad_prime.pow(4) * PRIMES[kicker];
            map.insert(product, rank);
            rank += 1;
        }
    }
    rank
}

fn generate_full_houses(map: &mut HashMap<u32, u16>, mut rank: u16) -> u16 {
    for trips_rank in (0..13).rev() {
        let trips_prime = PRIMES[trips_rank];

        for pair_rank in (0..13).rev() {
            if pair_rank == trips_rank {
                continue;
            }

            let product = trips_prime.pow(3) * PRIMES[pair_rank].pow(2);
            map.insert(product, rank);
            rank += 1;
        }
    }
    rank
}

fn generate_flushes(flush_lookup: &mut [u16], mut rank: u16) -> u16 {
    let mut combos = combinations(13, 5);
    combos.sort_by(|a, b| {
        let a_rev: Vec<_> = a.iter().copied().rev().collect();
        let b_rev: Vec<_> = b.iter().copied().rev().collect();
        b_rev.cmp(&a_rev)
    });

    for combo in combos {
        if is_straight_pattern(&combo) {
            continue;
        }

        let mut bits = 0u32;
        for &r in &combo {
            bits |= 1 << (r + 16);
        }

        flush_lookup[(bits >> 16) as usize] = rank;
        rank += 1;
    }
    rank
}

fn generate_straights(map: &mut HashMap<u32, u16>, mut rank: u16) -> u16 {
    let patterns = [
        [12, 11, 10, 9, 8], // A-K-Q-J-T
        [11, 10, 9, 8, 7],  // K-Q-J-T-9
        [10, 9, 8, 7, 6],   // Q-J-T-9-8
        [9, 8, 7, 6, 5],    // J-T-9-8-7
        [8, 7, 6, 5, 4],    // T-9-8-7-6
        [7, 6, 5, 4, 3],    // 9-8-7-6-5
        [6, 5, 4, 3, 2],    // 8-7-6-5-4
        [5, 4, 3, 2, 1],    // 7-6-5-4-3
        [4, 3, 2, 1, 0],    // 6-5-4-3-2
        [12, 3, 2, 1, 0],   // 5-4-3-2-A
    ];

    for pattern in patterns {
        let product: u32 = pattern.iter().map(|&r| PRIMES[r]).product();
        map.insert(product, rank);
        rank += 1;
    }
    rank
}

fn generate_three_of_kind(map: &mut HashMap<u32, u16>, mut rank: u16) -> u16 {
    for trips_rank in (0..13).rev() {
        let trips_prime = PRIMES[trips_rank];

        let mut kicker_combos = combinations(13, 2);
        kicker_combos.sort_by(|a, b| {
            let a_rev: Vec<_> = a.iter().copied().rev().collect();
            let b_rev: Vec<_> = b.iter().copied().rev().collect();
            b_rev.cmp(&a_rev)
        });

        for kickers in kicker_combos {
            if kickers.contains(&trips_rank) {
                continue;
            }

            let product = trips_prime.pow(3) * PRIMES[kickers[0]] * PRIMES[kickers[1]];
            map.insert(product, rank);
            rank += 1;
        }
    }
    rank
}

fn generate_two_pair(map: &mut HashMap<u32, u16>, mut rank: u16) -> u16 {
    let mut pair_combos = combinations(13, 2);
    pair_combos.sort_by(|a, b| {
        let a_rev: Vec<_> = a.iter().copied().rev().collect();
        let b_rev: Vec<_> = b.iter().copied().rev().collect();
        b_rev.cmp(&a_rev)
    });

    for pairs in pair_combos {
        let high = pairs[0].max(pairs[1]);
        let low = pairs[0].min(pairs[1]);

        for kicker in (0..13).rev() {
            if kicker == high || kicker == low {
                continue;
            }

            let product = PRIMES[high].pow(2) * PRIMES[low].pow(2) * PRIMES[kicker];
            map.insert(product, rank);
            rank += 1;
        }
    }
    rank
}

fn generate_one_pair(map: &mut HashMap<u32, u16>, mut rank: u16) -> u16 {
    for pair_rank in (0..13).rev() {
        let pair_prime = PRIMES[pair_rank];

        let mut kicker_combos = combinations(13, 3);
        kicker_combos.sort_by(|a, b| {
            let a_rev: Vec<_> = a.iter().copied().rev().collect();
            let b_rev: Vec<_> = b.iter().copied().rev().collect();
            b_rev.cmp(&a_rev)
        });

        for kickers in kicker_combos {
            if kickers.contains(&pair_rank) {
                continue;
            }

            let product =
                pair_prime.pow(2) * PRIMES[kickers[0]] * PRIMES[kickers[1]] * PRIMES[kickers[2]];
            map.insert(product, rank);
            rank += 1;
        }
    }
    rank
}

fn generate_high_card(map: &mut HashMap<u32, u16>, mut rank: u16) {
    let mut combos = combinations(13, 5);
    combos.sort_by(|a, b| {
        let a_rev: Vec<_> = a.iter().copied().rev().collect();
        let b_rev: Vec<_> = b.iter().copied().rev().collect();
        b_rev.cmp(&a_rev)
    });

    for combo in combos {
        if is_straight_pattern(&combo) {
            continue;
        }

        let product: u32 = combo.iter().map(|&r| PRIMES[r]).product();
        map.insert(product, rank);
        rank += 1;
    }
}