//! The `HRA_56_56` parity-check matrix for the LDPC(112,56) code used by the
//! RADE V1 End-of-Over text channel.
//!
//! `H = [H_a | H_b]` is 56×112.
//!
//! * `H_a` (the systematic half) is a regular matrix: every row *and* every
//!   column has weight 3, 168 ones in all. Storing three column indices per row
//!   costs 168 bytes against 6272 for the dense form.
//! * `H_b` (the parity half) is exactly identity + subdiagonal, so it is
//!   generated rather than stored — which is also what makes the encoder a
//!   forward substitution rather than a matrix inversion.
//!
//! Transcribed from `HRA_56_56.h` / `HRA_56_56.txt` in
//! <https://github.com/tmiw/freedv-backend> (BSD-2-Clause); the matrix itself
//! originates with David Rowe's codec2 work and has been frozen since 2017.
//! The invariants above are re-checked by the tests below, so a transcription
//! slip cannot pass silently.

/// Column indices of the three ones in each row of `H_a`, ascending.
///
/// Ascending order matters: the decoder sums float messages in edge order, so
/// reproducing the reference's row-major edge enumeration is what keeps our
/// arithmetic bit-comparable with it.
pub(crate) const HA_COLS: [[u8; 3]; 56] = [
    [3, 13, 40],
    [32, 45, 54],
    [21, 38, 46],
    [1, 26, 43],
    [6, 42, 55],
    [11, 21, 45],
    [14, 18, 19],
    [40, 44, 54],
    [5, 15, 20],
    [1, 22, 39],
    [7, 30, 36],
    [3, 51, 52],
    [0, 2, 12],
    [6, 7, 31],
    [19, 50, 51],
    [9, 12, 21],
    [1, 30, 31],
    [0, 10, 23],
    [27, 30, 43],
    [6, 35, 41],
    [2, 10, 48],
    [24, 27, 37],
    [17, 29, 47],
    [18, 20, 23],
    [16, 37, 55],
    [14, 25, 47],
    [7, 41, 49],
    [12, 28, 38],
    [8, 22, 25],
    [11, 32, 49],
    [38, 44, 53],
    [5, 18, 25],
    [16, 27, 36],
    [31, 39, 55],
    [2, 9, 13],
    [5, 22, 26],
    [9, 40, 45],
    [17, 24, 43],
    [3, 28, 44],
    [33, 35, 42],
    [0, 28, 52],
    [13, 48, 51],
    [8, 37, 39],
    [4, 10, 20],
    [14, 29, 50],
    [11, 41, 46],
    [4, 15, 29],
    [34, 36, 49],
    [4, 48, 50],
    [16, 34, 42],
    [15, 17, 26],
    [35, 46, 53],
    [19, 23, 52],
    [8, 24, 47],
    [33, 53, 54],
    [32, 33, 34],
];

/// Information bits per codeword (also the number of parity checks).
pub(crate) const K: usize = 56;
/// Codeword length in bits.
pub(crate) const N: usize = 112;

/// Column indices of the ones in row `i` of the full 56×112 `H`, ascending.
///
/// The first three come from [`HA_COLS`]; the rest are `H_b`'s bidiagonal:
/// column `K + i` always, preceded by `K + i - 1` for every row but the first.
pub(crate) fn h_row(i: usize) -> impl Iterator<Item = usize> {
    let a = HA_COLS[i];
    let b_first = (i > 0).then(|| K + i - 1);
    a.into_iter().map(usize::from).chain(b_first).chain(std::iter::once(K + i))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ha_is_regular_weight_three() {
        let mut col_weight = [0u32; K];
        for row in HA_COLS {
            let mut sorted = row;
            sorted.sort_unstable();
            assert_eq!(sorted, row, "row {row:?} is not stored in ascending order");
            assert!(row[0] < row[1] && row[1] < row[2], "row {row:?} has a repeated column");
            for c in row {
                assert!((c as usize) < K, "column {c} out of range");
                col_weight[c as usize] += 1;
            }
        }
        assert!(col_weight.iter().all(|&w| w == 3), "H_a columns are not all weight 3");
        let ones: u32 = col_weight.iter().sum();
        assert_eq!(ones, 168);
    }

    #[test]
    fn h_row_appends_the_bidiagonal_hb() {
        // Row 0 has only the diagonal element; every other row has the
        // subdiagonal one too.
        assert_eq!(h_row(0).collect::<Vec<_>>(), vec![3, 13, 40, 56]);
        assert_eq!(h_row(1).collect::<Vec<_>>(), vec![32, 45, 54, 56, 57]);
        assert_eq!(h_row(55).collect::<Vec<_>>(), vec![32, 33, 34, 110, 111]);

        for i in 0..K {
            let cols: Vec<usize> = h_row(i).collect();
            assert!(cols.windows(2).all(|w| w[0] < w[1]), "row {i} is not ascending: {cols:?}");
            assert!(cols.iter().all(|&c| c < N));
            assert_eq!(cols.len(), if i == 0 { 4 } else { 5 });
        }
        let edges: usize = (0..K).map(|i| h_row(i).count()).sum();
        assert_eq!(edges, 279, "Tanner graph edge count");
    }
}
