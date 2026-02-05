//! Combinatorics utilities for poker hand evaluation.

/// All 21 ways to choose 5 cards from 7.
pub const FIVE_FROM_SEVEN: [[usize; 5]; 21] = [
    [0, 1, 2, 3, 4],
    [0, 1, 2, 3, 5],
    [0, 1, 2, 3, 6],
    [0, 1, 2, 4, 5],
    [0, 1, 2, 4, 6],
    [0, 1, 2, 5, 6],
    [0, 1, 3, 4, 5],
    [0, 1, 3, 4, 6],
    [0, 1, 3, 5, 6],
    [0, 1, 4, 5, 6],
    [0, 2, 3, 4, 5],
    [0, 2, 3, 4, 6],
    [0, 2, 3, 5, 6],
    [0, 2, 4, 5, 6],
    [0, 3, 4, 5, 6],
    [1, 2, 3, 4, 5],
    [1, 2, 3, 4, 6],
    [1, 2, 3, 5, 6],
    [1, 2, 4, 5, 6],
    [1, 3, 4, 5, 6],
    [2, 3, 4, 5, 6],
];

/// Compute the binomial coefficient C(n, k).
///
/// Returns 0 when k > n. Uses symmetry so C(n, k) == C(n, n - k).
///
/// # Examples
///
/// ```
/// assert_eq!(binomial(5, 2), 10);
/// assert_eq!(binomial(6, 3), 20);
/// assert_eq!(binomial(4, 5), 0);
/// ```
#[must_use] 
pub fn binomial(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }

    let mut res = 1;
    let k = k.min(n - k); // C(n,k) = C(n,n-k)

    for i in 0..k {
        res = res * (n - i) / (i + 1);
    }

    res
}

/// Generate all k-sized combinations of indices 0..n-1.
///
/// Returns an empty vector if k > n. If k == 0, returns a vector containing a single empty combination.
/// Each combination is a Vec<usize> of length k with indices in ascending order.
///
/// # Examples
///
/// ```
/// let combos = combinations(4, 2);
/// assert_eq!(combos.len(), 6);
/// assert!(combos.contains(&vec![0, 1]));
/// assert!(combos.contains(&vec![2, 3]));
/// ```
#[must_use] 
pub fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    if k > n {
        return Vec::new();
    }
    if k == 0 {
        return vec![Vec::new()];
    }

    let mut result = Vec::with_capacity(binomial(n, k));
    let mut indices: Vec<usize> = (0..k).collect();

    loop {
        result.push(indices.clone());

        // Find rightmost index that can be incremented
        let mut i = k;
        while i > 0 {
            i -= 1;
            if indices[i] != i + n - k {
                break;
            }
        }

        if indices[i] == i + n - k {
            break; // Done
        }

        indices[i] += 1;
        for j in (i + 1)..k {
            indices[j] = indices[j - 1] + 1;
        }
    }

    result
}

/// Determine whether five card ranks form a straight, including the wheel (A-2-3-4-5).
///
/// Expects a slice of exactly five rank indices. Ranks use 0..=12 with `12` representing Ace;
/// the special case `[0, 1, 2, 3, 12]` is treated as a valid straight (wheel).
///
/// # Examples
///
/// ```
/// // 8-9-10-J-Q (consecutive)
/// assert!(is_straight_pattern(&[6, 7, 8, 9, 10]));
/// // Wheel: A-2-3-4-5
/// assert!(is_straight_pattern(&[0, 1, 2, 3, 12]));
/// // Not a straight: gap and duplicate
/// assert!(!is_straight_pattern(&[0, 1, 2, 3, 5]));
/// assert!(!is_straight_pattern(&[0, 0, 1, 2, 3]));
/// ```
///
/// # Returns
///
/// `true` if the five ranks form a straight (including the wheel), `false` otherwise.
#[must_use] 
pub fn is_straight_pattern(ranks: &[usize]) -> bool {
    if ranks.len() != 5 {
        return false;
    }

    let mut sorted = ranks.to_vec();
    sorted.sort_unstable();

    // Check wheel (A-2-3-4-5)
    if sorted == vec![0, 1, 2, 3, 12] {
        return true;
    }
    // Check normal straight (5 consecutive, strictly increasing)
    sorted.windows(2).all(|w| w[1] == w[0] + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binomial() {
        assert_eq!(binomial(5, 0), 1);
        assert_eq!(binomial(5, 1), 5);
        assert_eq!(binomial(5, 2), 10);
        assert_eq!(binomial(5, 5), 1);
        assert_eq!(binomial(7, 5), 21);
        assert_eq!(binomial(13, 5), 1287);
    }

    #[test]
    fn test_combinations() {
        let c = combinations(4, 2);
        assert_eq!(c.len(), 6);
        assert!(c.contains(&vec![0, 1]));
        assert!(c.contains(&vec![2, 3]));
    }

    #[test]
    fn test_five_from_seven() {
        assert_eq!(FIVE_FROM_SEVEN.len(), 21);
        // Verify each combo has 5 unique indices
        for combo in FIVE_FROM_SEVEN {
            let mut sorted = combo.to_vec();
            sorted.sort();
            sorted.dedup();
            assert_eq!(sorted.len(), 5);
        }
    }

    #[test]
    fn test_is_straight_pattern() {
        assert!(is_straight_pattern(&[8, 9, 10, 11, 12])); // A-high straight
        assert!(is_straight_pattern(&[0, 1, 2, 3, 4])); // 6-high straight
        assert!(is_straight_pattern(&[0, 1, 2, 3, 12])); // Wheel (A-2-3-4-5)
        assert!(!is_straight_pattern(&[0, 1, 2, 3, 5])); // Gap
        assert!(!is_straight_pattern(&[0, 0, 1, 2, 3])); // Duplicate
    }
}