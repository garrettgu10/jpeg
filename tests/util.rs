use jpeg::color::*;
use jpeg::zz::*;

#[test]
fn rgb_to_ycbcr() {
    assert_eq!(YCbCr{
        y: 40,
        cb: 161,
        cr: 138,
    }, RGB {
        r: 55,
        g: 22,
        b: 100,
    }.ycbcr())
}

#[test]
fn rcbcr_to_rgb() {
    assert_eq!(RGB {
        r: 54,
        g: 21,
        b: 98,
    }, YCbCr{
        y: 40,
        cb: 161,
        cr: 138,
    }.rgb())
}

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