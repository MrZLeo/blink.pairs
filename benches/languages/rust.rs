use crate::r#const::*;
use std::ops::Not;
use std::simd::num::SimdUint;
use std::simd::{cmp::*, SimdElement};
use std::simd::{Mask, Simd};

pub trait SimdNum<const L: usize>:
    Sized
    + Copy
    + std::fmt::Debug
    + std::simd::SimdElement
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
    + std::convert::From<u8>
    + std::convert::Into<u16>
    + std::convert::Into<usize>
    + std::cmp::PartialEq
    + std::cmp::PartialOrd
where
    std::simd::LaneCount<L>: std::simd::SupportedLaneCount,
{
    const ZERO: Self;
    const ZERO_VEC: Simd<Self, L>;

    // Delimiters
    const SPACE_DELIMITER: Simd<Self, L>;
    const SLASH_DELIMITER: Simd<Self, L>;
    const DOT_DELIMITER: Simd<Self, L>;
    const COMMA_DELIMITER: Simd<Self, L>;
    const UNDERSCORE_DELIMITER: Simd<Self, L>;
    const DASH_DELIMITER: Simd<Self, L>;
    const COLON_DELIMITER: Simd<Self, L>;
    const DELIMITER_BONUS: Simd<Self, L>;

    // Capitalization
    const CAPITAL_START: Simd<Self, L>;
    const CAPITAL_END: Simd<Self, L>;
    const TO_LOWERCASE_MASK: Simd<Self, L>;

    // Scoring Params
    const CAPITALIZATION_BONUS: Simd<Self, L>;
    const MATCHING_CASE_BONUS: Simd<Self, L>;

    const GAP_OPEN_PENALTY: Simd<Self, L>;
    const GAP_EXTEND_PENALTY: Simd<Self, L>;
    const MATCH_SCORE: Simd<Self, L>;
    const MISMATCH_PENALTY: Simd<Self, L>;
    const PREFIX_MATCH_SCORE: Simd<Self, L>;

    fn from_usize(n: usize) -> Self;
}

pub trait SimdVec<N: SimdNum<L>, const L: usize>:
    Sized
    + Copy
    + std::ops::Add<Output = Simd<N, L>>
    + std::ops::BitOr<Output = Simd<N, L>>
    + std::simd::cmp::SimdPartialEq<Mask = Mask<<N as SimdElement>::Mask, L>>
    + std::simd::cmp::SimdOrd
    + std::simd::num::SimdUint
where
    N: SimdNum<L>,
    N::Mask: std::simd::MaskElement,
    std::simd::LaneCount<L>: std::simd::SupportedLaneCount,
{
}

pub trait SimdMask<N: SimdNum<L>, const L: usize>:
    Sized
    + Copy
    + std::ops::Not<Output = Mask<N::Mask, L>>
    + std::ops::BitAnd<Output = Mask<N::Mask, L>>
    + std::ops::BitOr<Output = Mask<N::Mask, L>>
    + std::simd::cmp::SimdPartialEq<Mask = Mask<<N as SimdElement>::Mask, L>>
where
    std::simd::LaneCount<L>: std::simd::SupportedLaneCount,
{
}

macro_rules! simd_num_impl {
    ($type:ident,$($lanes:literal),+) => {
        $(
            impl SimdNum<$lanes> for $type {
                const ZERO: Self = 0;
                const ZERO_VEC: Simd<Self, $lanes> = Simd::from_array([0; $lanes]);

                const SPACE_DELIMITER: Simd<Self, $lanes> = Simd::from_array([b' ' as $type; $lanes]);
                const SLASH_DELIMITER: Simd<Self, $lanes> = Simd::from_array([b'/' as $type; $lanes]);
                const DOT_DELIMITER: Simd<Self, $lanes> = Simd::from_array([b'.' as $type; $lanes]);
                const COMMA_DELIMITER: Simd<Self, $lanes> = Simd::from_array([b',' as $type; $lanes]);
                const UNDERSCORE_DELIMITER: Simd<Self, $lanes> = Simd::from_array([b'_' as $type; $lanes]);
                const DASH_DELIMITER: Simd<Self, $lanes> = Simd::from_array([b'-' as $type; $lanes]);
                const COLON_DELIMITER: Simd<Self, $lanes> = Simd::from_array([b':' as $type; $lanes]);
                const DELIMITER_BONUS: Simd<Self, $lanes> = Simd::from_array([DELIMITER_BONUS as $type; $lanes]);

                const CAPITAL_START: Simd<Self, $lanes> = Simd::from_array([b'A' as $type; $lanes]);
                const CAPITAL_END: Simd<Self, $lanes> = Simd::from_array([b'Z' as $type; $lanes]);
                const TO_LOWERCASE_MASK: Simd<Self, $lanes> = Simd::from_array([0x20; $lanes]);

                const CAPITALIZATION_BONUS: Simd<Self, $lanes> = Simd::from_array([CAPITALIZATION_BONUS as $type; $lanes]);
                const MATCHING_CASE_BONUS: Simd<Self, $lanes> = Simd::from_array([MATCHING_CASE_BONUS as $type; $lanes]);

                const GAP_OPEN_PENALTY: Simd<Self, $lanes> = Simd::from_array([GAP_OPEN_PENALTY as $type; $lanes]);
                const GAP_EXTEND_PENALTY: Simd<Self, $lanes> = Simd::from_array([GAP_EXTEND_PENALTY as $type; $lanes]);
                const MATCH_SCORE: Simd<Self, $lanes> = Simd::from_array([MATCH_SCORE as $type; $lanes]);
                const MISMATCH_PENALTY: Simd<Self, $lanes> = Simd::from_array([MISMATCH_PENALTY as $type; $lanes]);
                const PREFIX_MATCH_SCORE: Simd<Self, $lanes> = Simd::from_array([(MATCH_SCORE + PREFIX_BONUS) as $type; $lanes]);

                #[inline(always)]
                fn from_usize(n: usize) -> Self {
                    n as $type
                }
            }
            impl SimdVec<$type, $lanes> for Simd<$type, $lanes> {}
            impl SimdMask<$type, $lanes> for Mask<<$type as SimdElement>::Mask, $lanes> {}
        )+
    };
}
simd_num_impl!(u8, 1, 2, 4, 8, 16, 32, 64);
simd_num_impl!(u16, 1, 2, 4, 8, 16, 32);

#[inline(always)]
pub(crate) fn simd_to_lowercase_with_mask<N, const L: usize>(
    data: Simd<N, L>,
) -> (Mask<N::Mask, L>, Simd<N, L>)
where
    N: SimdNum<L>,
    std::simd::LaneCount<L>: std::simd::SupportedLaneCount,
    Simd<N, L>: SimdVec<N, L>,
    Mask<N::Mask, L>: SimdMask<N, L>,
{
    let is_capital_mask: Mask<N::Mask, L> =
        data.simd_ge(N::CAPITAL_START) & data.simd_le(N::CAPITAL_END);
    let lowercase = data | is_capital_mask.select(N::TO_LOWERCASE_MASK, N::ZERO_VEC);
    (is_capital_mask, lowercase)
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct NeedleChar<N, const L: usize>
where
    N: SimdNum<L>,
    std::simd::LaneCount<L>: std::simd::SupportedLaneCount,
{
    pub(crate) lowercase: Simd<N, L>,
    pub(crate) is_capital_mask: Mask<N::Mask, L>,
}
impl<N, const L: usize> NeedleChar<N, L>
where
    N: SimdNum<L>,
    std::simd::LaneCount<L>: std::simd::SupportedLaneCount,
    Simd<N, L>: SimdVec<N, L>,
    Mask<N::Mask, L>: SimdMask<N, L>,
{
    #[inline(always)]
    pub(crate) fn new(char: N) -> Self {
        let (is_capital_mask, lowercase) = simd_to_lowercase_with_mask::<N, L>(Simd::splat(char));
        Self {
            lowercase,
            is_capital_mask,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct HaystackChar<N, const L: usize>
where
    N: SimdNum<L>,
    std::simd::LaneCount<L>: std::simd::SupportedLaneCount,
{
    pub(crate) lowercase: Simd<N, L>,
    pub(crate) is_capital_mask: Mask<N::Mask, L>,
    pub(crate) is_delimiter_mask: Mask<N::Mask, L>,
}
impl<N, const L: usize> HaystackChar<N, L>
where
    N: SimdNum<L>,
    std::simd::LaneCount<L>: std::simd::SupportedLaneCount,
    Simd<N, L>: SimdVec<N, L>,
    Mask<N::Mask, L>: SimdMask<N, L>,
{
    #[inline(always)]
    pub(crate) fn new(chars: Simd<N, L>) -> Self {
        let (is_capital_mask, lowercase) = simd_to_lowercase_with_mask::<N, L>(chars);
        let is_delimiter_mask: Mask<N::Mask, L> = N::SPACE_DELIMITER.simd_eq(lowercase)
            | N::SLASH_DELIMITER.simd_eq(lowercase)
            | N::DOT_DELIMITER.simd_eq(lowercase)
            | N::COMMA_DELIMITER.simd_eq(lowercase)
            | N::UNDERSCORE_DELIMITER.simd_eq(lowercase)
            | N::DASH_DELIMITER.simd_eq(lowercase)
            | N::SPACE_DELIMITER.simd_eq(lowercase);
        Self {
            lowercase,
            is_capital_mask,
            is_delimiter_mask,
        }
    }

    #[inline(always)]
    pub(crate) fn from_haystacks(haystacks: &[&str; L], i: usize) -> Self {
        // Convert haystacks to a static array of bytes chunked for SIMD
        let chars = std::array::from_fn(|j| {
            N::from(*haystacks[j].as_bytes().get(i).to_owned().unwrap_or(&0))
        });
        // pre-compute haystack case mask, delimiter mask, and lowercase
        HaystackChar::new(Simd::from_array(chars))
    }
}

impl<N, const L: usize> Default for HaystackChar<N, L>
where
    N: SimdNum<L>,
    std::simd::LaneCount<L>: std::simd::SupportedLaneCount,
    Simd<N, L>: SimdVec<N, L>,
    Mask<N::Mask, L>: SimdMask<N, L>,
{
    fn default() -> Self {
        Self {
            lowercase: N::ZERO_VEC,
            is_capital_mask: Mask::splat(false),
            is_delimiter_mask: Mask::splat(false),
        }
    }
}

#[inline(always)]
pub(crate) fn smith_waterman_inner<N, const L: usize>(
    width: usize,
    needle_char: NeedleChar<N, L>,
    haystack: &[HaystackChar<N, L>],
    prev_score_col: Option<&[Simd<N, L>]>,
    curr_score_col: &mut [Simd<N, L>],
) where
    N: SimdNum<L>,
    std::simd::LaneCount<L>: std::simd::SupportedLaneCount,
    Simd<N, L>: SimdVec<N, L>,
    Mask<N::Mask, L>: SimdMask<N, L>,
{
    let mut up_score_simd = N::ZERO_VEC;
    let mut up_gap_penalty_mask = Mask::splat(true);
    let mut left_gap_penalty_mask = Mask::splat(true);
    let mut delimiter_bonus_enabled_mask = Mask::splat(false);

    for haystack_idx in 0..width {
        let haystack_char = haystack[haystack_idx];

        let (diag, left) = if haystack_idx == 0 {
            (N::ZERO_VEC, N::ZERO_VEC)
        } else {
            prev_score_col
                .map(|c| (c[haystack_idx - 1], c[haystack_idx]))
                .unwrap_or((N::ZERO_VEC, N::ZERO_VEC))
        };

        // Calculate diagonal (match/mismatch) scores
        let match_mask: Mask<N::Mask, L> = needle_char.lowercase.simd_eq(haystack_char.lowercase);
        let matched_casing_mask: Mask<N::Mask, L> = needle_char
            .is_capital_mask
            .simd_eq(haystack_char.is_capital_mask);
        let diag_score: Simd<N, L> = match_mask.select(
            diag + matched_casing_mask.select(N::MATCHING_CASE_BONUS, N::ZERO_VEC)
                + if haystack_idx > 0 {
                    let prev_haystack_char = haystack[haystack_idx - 1];

                    // ignore capitalization on the prefix
                    let capitalization_bonus_mask: Mask<N::Mask, L> =
                        haystack_char.is_capital_mask & prev_haystack_char.is_capital_mask.not();
                    let capitalization_bonus =
                        capitalization_bonus_mask.select(N::CAPITALIZATION_BONUS, N::ZERO_VEC);

                    let delimiter_bonus_mask: Mask<N::Mask, L> =
                        prev_haystack_char.is_delimiter_mask & delimiter_bonus_enabled_mask;
                    let delimiter_bonus =
                        delimiter_bonus_mask.select(N::DELIMITER_BONUS, N::ZERO_VEC);

                    capitalization_bonus + delimiter_bonus + N::MATCH_SCORE
                } else {
                    // Give a bonus for prefix matches
                    N::PREFIX_MATCH_SCORE
                },
            diag.saturating_sub(N::MISMATCH_PENALTY),
        );

        // Load and calculate up scores (skipping char in haystack)
        let up_gap_penalty = up_gap_penalty_mask.select(N::GAP_OPEN_PENALTY, N::GAP_EXTEND_PENALTY);
        let up_score = up_score_simd.saturating_sub(up_gap_penalty);

        // Load and calculate left scores (skipping char in needle)
        let left_gap_penalty =
            left_gap_penalty_mask.select(N::GAP_OPEN_PENALTY, N::GAP_EXTEND_PENALTY);
        let left_score = left.saturating_sub(left_gap_penalty);

        // Calculate maximum scores
        let max_score = diag_score.simd_max(up_score).simd_max(left_score);

        // Update gap penalty mask
        let diag_mask: Mask<N::Mask, L> = max_score.simd_eq(diag_score);
        up_gap_penalty_mask = max_score.simd_ne(up_score) | diag_mask;
        left_gap_penalty_mask = max_score.simd_ne(left_score) | diag_mask;

        // Only enable delimiter bonus if we've seen a non-delimiter char
        delimiter_bonus_enabled_mask |= haystack_char.is_delimiter_mask.not();

        // Store the scores for the next iterations
        up_score_simd = max_score;
        curr_score_col[haystack_idx] = max_score;

        // Store the maximum score across all runs
    }
}

#[inline]
pub fn smith_waterman<N, const W: usize, const L: usize>(
    needle: &str,
    haystacks: &[&str; L],
) -> ([u16; L], Vec<[Simd<N, L>; W]>, [bool; L])
where
    N: SimdNum<L>,
    std::simd::LaneCount<L>: std::simd::SupportedLaneCount,
    Simd<N, L>: SimdVec<N, L>,
    Mask<N::Mask, L>: SimdMask<N, L>,
{
    let needle_str = needle;
    let needle = needle.as_bytes();

    let haystack: [HaystackChar<N, L>; W] =
        std::array::from_fn(|i| HaystackChar::from_haystacks(haystacks, i));

    // State
    let mut score_matrix = vec![[N::ZERO_VEC; W]; needle.len()];

    for needle_idx in 0..needle.len() {
        let needle_char = NeedleChar::new(N::from(needle[needle_idx]));

        let (prev_score_col, curr_score_col) = if needle_idx == 0 {
            (None, &mut score_matrix[needle_idx])
        } else {
            let (a, b) = score_matrix.split_at_mut(needle_idx);
            (Some(a[needle_idx - 1].as_slice()), &mut b[0])
        };

        smith_waterman_inner(W, needle_char, &haystack, prev_score_col, curr_score_col);
    }

    let exact_matches = std::array::from_fn(|i| haystacks[i] == needle_str);

    let mut all_time_max_score = N::ZERO_VEC;
    for score_col in score_matrix.iter() {
        for score in score_col {
            all_time_max_score = score.simd_max(all_time_max_score);
        }
    }

    let max_scores_vec = std::array::from_fn(|i| {
        let mut score = all_time_max_score[i].into();
        if exact_matches[i] {
            score += EXACT_MATCH_BONUS;
        }
        score
    });

    (max_scores_vec, score_matrix, exact_matches)
}

#[inline]
pub fn typos_from_score_matrix<N, const W: usize, const L: usize>(
    score_matrix: &[[Simd<N, L>; W]],
) -> [u16; L]
where
    N: SimdNum<L>,
    std::simd::LaneCount<L>: std::simd::SupportedLaneCount,
    Simd<N, L>: SimdVec<N, L>,
    Mask<N::Mask, L>: SimdMask<N, L>,
{
    let mut typo_count = [0u16; L];
    let mut scores = N::ZERO_VEC;
    let mut positions = N::ZERO_VEC;

    // Get the starting position by looking at the last column
    // (last character of the needle)
    let last_column = score_matrix.last().unwrap();
    for (idx, &row_scores) in last_column.iter().enumerate() {
        let row_max_mask: Mask<N::Mask, L> = row_scores.simd_gt(scores);
        scores = row_max_mask.select(row_scores, scores);
        positions = row_max_mask.select(Simd::splat(N::from_usize(idx)), positions);
    }

    // Traceback and store the matched indices
    for (idx, &row_idx) in positions.to_array().iter().enumerate() {
        let mut col_idx = score_matrix.len() - 1;
        let mut row_idx: usize = row_idx.into();
        let mut score = scores[idx];

        // NOTE: row_idx = 0 or col_idx = 0 will always have a score of 0
        while col_idx > 0 {
            // Must be moving left
            if row_idx == 0 {
                typo_count[idx] += 1;
                col_idx -= 1;
                continue;
            }

            // Gather up the scores for all possible paths
            let diag = score_matrix[col_idx - 1][row_idx - 1][idx];
            let left = score_matrix[col_idx - 1][row_idx][idx];
            let up = score_matrix[col_idx][row_idx - 1][idx];

            // Match or mismatch
            if diag >= left && diag >= up {
                // Must be a mismatch
                if diag >= score {
                    typo_count[idx] += 1;
                }
                row_idx -= 1;
                col_idx -= 1;
                score = diag;
            // Skipped character in needle
            } else if left >= up {
                typo_count[idx] += 1;
                col_idx -= 1;
                score = left;
            // Skipped character in haystack
            } else {
                row_idx -= 1;
                score = up;
            }
        }

        // HACK: Compensate for the last column being a typo
        if col_idx == 0 && score == N::ZERO {
            typo_count[idx] += 1;
        }
    }

    typo_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHAR_SCORE: u16 = MATCH_SCORE + MATCHING_CASE_BONUS;

    fn get_score(needle: &str, haystack: &str) -> u16 {
        smith_waterman::<u8, 16, 1>(needle, &[haystack; 1]).0[0]
    }

    fn get_typos(needle: &str, haystack: &str) -> u16 {
        typos_from_score_matrix(&smith_waterman::<u8, 4, 1>(needle, &[haystack; 1]).1)[0]
    }

    #[test]
    fn test_score_basic() {
        assert_eq!(get_score("b", "abc"), CHAR_SCORE);
        assert_eq!(get_score("c", "abc"), CHAR_SCORE);
    }

    #[test]
    fn test_typos_basic() {
        assert_eq!(get_typos("a", "abc"), 0);
        assert_eq!(get_typos("b", "abc"), 0);
        assert_eq!(get_typos("c", "abc"), 0);
        assert_eq!(get_typos("ac", "abc"), 0);

        assert_eq!(get_typos("d", "abc"), 1);
        assert_eq!(get_typos("da", "abc"), 1);
        assert_eq!(get_typos("dc", "abc"), 1);
        assert_eq!(get_typos("ad", "abc"), 1);
        assert_eq!(get_typos("adc", "abc"), 1);
        assert_eq!(get_typos("add", "abc"), 2);
        assert_eq!(get_typos("ddd", "abc"), 3);
        assert_eq!(get_typos("ddd", ""), 3);
        assert_eq!(get_typos("d", ""), 1);
    }

    #[test]
    fn test_score_prefix() {
        assert_eq!(get_score("a", "abc"), CHAR_SCORE + PREFIX_BONUS);
        assert_eq!(get_score("a", "aabc"), CHAR_SCORE + PREFIX_BONUS);
        assert_eq!(get_score("a", "babc"), CHAR_SCORE);
    }

    #[test]
    fn test_score_exact_match() {
        assert_eq!(
            get_score("a", "a"),
            CHAR_SCORE + EXACT_MATCH_BONUS + PREFIX_BONUS
        );
        assert_eq!(
            get_score("abc", "abc"),
            3 * CHAR_SCORE + EXACT_MATCH_BONUS + PREFIX_BONUS
        );
        assert_eq!(get_score("ab", "abc"), 2 * CHAR_SCORE + PREFIX_BONUS);
        // assert_eq!(run_single("abc", "ab"), 2 * CHAR_SCORE + PREFIX_BONUS);
    }

    #[test]
    fn test_score_delimiter() {
        assert_eq!(get_score("b", "a-b"), CHAR_SCORE + DELIMITER_BONUS);
        assert_eq!(get_score("a", "a-b-c"), CHAR_SCORE + PREFIX_BONUS);
        assert_eq!(get_score("b", "a--b"), CHAR_SCORE + DELIMITER_BONUS);
        assert_eq!(get_score("c", "a--bc"), CHAR_SCORE);
        assert_eq!(get_score("a", "-a--bc"), CHAR_SCORE);
        assert_eq!(get_score("-", "a-bc"), CHAR_SCORE);
        assert_eq!(get_score("-", "a--bc"), CHAR_SCORE + DELIMITER_BONUS);
    }

    #[test]
    fn test_score_affine_gap() {
        assert_eq!(
            get_score("test", "Uterst"),
            CHAR_SCORE * 4 - GAP_OPEN_PENALTY
        );
        assert_eq!(
            get_score("test", "Uterrst"),
            CHAR_SCORE * 4 - GAP_OPEN_PENALTY - GAP_EXTEND_PENALTY
        );
    }

    #[test]
    fn test_score_capital_bonus() {
        assert_eq!(get_score("a", "A"), MATCH_SCORE + PREFIX_BONUS);
        assert_eq!(get_score("A", "Aa"), CHAR_SCORE + PREFIX_BONUS);
        assert_eq!(get_score("D", "forDist"), CHAR_SCORE + CAPITALIZATION_BONUS);
        assert_eq!(get_score("D", "foRDist"), CHAR_SCORE);
    }

    #[test]
    fn test_score_prefix_beats_delimiter() {
        assert!(get_score("swap", "swap(test)") > get_score("swap", "iter_swap(test)"));
        assert!(get_score("_", "_private_member") > get_score("_", "public_member"));
    }
}
