fn main() {
    println!("Hello, world!");
}


#[test]
fn while_loop(){
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter: {}", counter)
        }
        counter += 1;
    }
}
