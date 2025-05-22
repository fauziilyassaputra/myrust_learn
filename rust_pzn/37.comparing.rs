
fn main() {
    println!("Hello, world!");
}

struct Apple{
    quantity: i32,
}

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

use std::cmp::Ordering;

impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

#[test]
fn test_comparing(){
    let apple1 = Apple{quantity: 10};
    let apple2 = Apple{quantity: 20};

    println!("apple1 == apple2 : {}", apple1 == apple2);
    println!("apple1 < apple2 : {}", apple1 < apple2);
    println!("apple1 > apple2 : {}", apple1 > apple2);

}

