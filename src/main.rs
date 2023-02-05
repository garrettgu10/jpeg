use jpeg;
fn main() {
    println!("Hello, world!");
    jpeg::test::aaa();
    println!("{:?}", jpeg::color::RGB{r: 23, g: 55, b: 22}.ycbcr());
    println!("{:?}", jpeg::zz::zz_to_idx(1));
    println!("{:?}", jpeg::zz::idx_to_zz(0, 1));
}
