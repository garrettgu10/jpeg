use jpeg;
fn main() {
    println!("Hello, world!");
    jpeg::test::aaa();
    println!("{:?}", jpeg::color::RGB{r: 23, g: 55, b: 22}.ycbcr());
}
