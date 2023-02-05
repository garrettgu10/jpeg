use jpeg::color::{self, YCbCr, RGB};

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