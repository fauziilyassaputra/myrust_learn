fn main() {
    println!("Hello, world!");
}

#[test]
fn comparison_operator(){
    let a = 10;
    let b = 20;
    let c: bool = a > b;
    println!("10 > 20: {}", c);
    let d = a < b;
    println!("10 < 20: {}", d);
    
    let number_1 = 200;
    let number_2 = 200;
    let result = number_1 >= number_2;
    println!("200 >= 200: {}", result);

}
