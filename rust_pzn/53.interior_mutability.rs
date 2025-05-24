fn main() {
    println!("Hello, world!");
}

/*
ATURAN RefCall<T> :
- banyak imutable arraw diperbolehkan
- satu mutable arraw diperbolehkan
- banyak mutable borrow tidak diperbolehkan
- sekaligus mutable dan immutable borrow tidak diperbolehkan
*/

use std::cell::RefCell;
use std::cell::RefMut;

#[derive(Debug)]
struct Seller {
    name: RefCell<String>,
    active: RefCell<bool>
}

#[test]
fn test_ref_cell() {
    let seller = Seller{
        name: RefCell::new("hoshimi".to_string()),
        active: RefCell::new(true)
    };

    let mut result = seller.name.borrow_mut();
    *result = "yanagi".to_string();

    println!("{:?}", seller);
}
