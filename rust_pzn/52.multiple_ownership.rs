fn main() {
    println!("Hello, world!");
}

/*
- default di rust adalah bahwa satu value hanya bisa dimiliki oleh satu variable
- jika ingin membuat  satu value bisa dimiliki oleh beberapa variable, kita harus menggunakan type Rc<T> (Reference Counted)

-Rc<T> adalah tipe data smart pointer jadi penggunaannya mirip seperti Box<T>
*/

use std::rc::Rc;

enum Brand {
    Of(String, Rc<Brand>),
    End
}

#[test]
fn test_multiple_ownership(){
    let apple = Rc::new(Brand::Of("apple".to_string(), Rc::new(Brand::End)));
    println!("Apple reference count: {}", Rc::strong_count(&apple));

    let laptop = Brand::Of("laptop".to_string(), Rc::clone(&apple));
    println!("apple reference count: {}", Rc::strong_count(&apple));
    {
    let smartphone = Brand::Of("laptop".to_string(), Rc::clone(&apple));
    println!("apple reference count: {}", Rc::strong_count(&apple));
    }

    // pada scope ini hanya tersisa 2 reference count karena variable smartphone sudah selesai
    println!("Apple reference count: {}", Rc::strong_count(&apple));
}
