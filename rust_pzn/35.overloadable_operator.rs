fn main() {
    println!("Hello, world!");
}

use core::ops::Add;

struct Apple{
    quantity: i32,
}
impl Add for Apple {
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output {
        Apple {
            quantity: self.quantity + rhs.quantity
        }
    }
}

#[test]
fn test_operator_add(){
    let apple1 = Apple{quantity: 10};
    let apple2 = Apple{quantity: 10};

    let apple3 =  apple1 + apple2;
    println!("{}", apple3.quantity);
}
