fn main() {
    println!("Hello, world!");
}


struct Category{
    id: String,
    name: String
}

use std::fmt::{Debug, Formatter};

impl Debug for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("category")
        .field("id", &self.id)
        .field("name",  &self.name)
        .finish()
    }
}

#[test]
fn test_format(){
    let category = Category{
        name: String::from("Gadget"),
        id: String::from("GADGET")
    };
    //  memakai {:?} karena debug dan bukan display
    println!("{:?}", category);
}
