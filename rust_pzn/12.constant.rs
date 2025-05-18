fn main() {
    println!("Hello, world!");
}

// const must be uppercase
const MAXIMUM: i32 = 100;

#[test]
fn my_constant(){
    const MINIMUM: i32 = 0;
    println!("minimum : {},maximum: {}", MINIMUM, MAXIMUM);
}
