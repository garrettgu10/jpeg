static ZZ: [(usize, usize); 64] = [
    (0, 0),
    (0, 1), (1, 0),         
    (2, 0), (1, 1), (0, 2),
    (0, 3), (1, 2), (2, 1), (3, 0),
    (4, 0), (3, 1), (2, 2), (1, 3), (0, 4),
    (0, 5), (1, 4), (2, 3), (3, 2), (4, 1), (5, 0),
    (6, 0), (5, 1), (4, 2), (3, 3), (2, 4), (1, 5), (0, 6),
    (0, 7), (1, 6), (2, 5), (3, 4), (4, 3), (5, 2), (6, 1), (7, 0),
    (7, 1), (6, 2), (5, 3), (4, 4), (3, 5), (2, 6), (1, 7),
    (2, 7), (3, 6), (4, 5), (5, 4), (6, 3), (7, 2),
    (7, 3), (6, 4), (5, 5), (4, 6), (3, 7),
    (4, 7), (5, 6), (6, 5), (7, 4),
    (7, 5), (6, 6), (5, 7),
    (6, 7), (7, 6),
    (7, 7),
];

static IDX: [[usize; 8]; 8] = [
    [  0,  1,  5,  6, 14, 15, 27, 28 ],
    [  2,  4,  7, 13, 16, 26, 29, 42 ],
    [  3,  8, 12, 17, 25, 30, 41, 43 ],
    [  9, 11, 18, 24, 31, 40, 44, 53 ],
    [ 10, 19, 23, 32, 39, 45, 52, 54 ],
    [ 20, 22, 33, 38, 46, 51, 55, 60 ],
    [ 21, 34, 37, 47, 50, 56, 59, 61 ],
    [ 35, 36, 48, 49, 57, 58, 62, 63 ],
];

pub fn zz_to_idx(zz: usize) -> (usize, usize) {
    return ZZ[zz];
}

pub fn idx_to_zz(i: usize, j: usize) -> usize {
    return IDX[i][j];
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn bij() {
        for i in 0..64 {
            let idx = zz_to_idx(i);
            assert_eq!(idx_to_zz(idx.0, idx.1), i);
        }

        for i in 0..8 {
            for j in 0..8 {
                assert_eq!(zz_to_idx(idx_to_zz(i, j)), (i, j));
            }
        }
    }
}