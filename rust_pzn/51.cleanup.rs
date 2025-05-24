fn main() {
    println!("Hello, world!");
}

struct Book{
    title: String
}

/*
- Drop trait merupakan Trait yang bisa kita implementasikan untuk membuat kode yang akan dieksekusi 
sebelum value di drop
*/

impl Drop for Book {
    fn drop(&mut self){
        println!("Dropping book: {}", self.title)
    }
}

#[test]
fn test_drop(){
    let book = Book{title: "Rust Programming".to_string()};
    println!("{}", book.title)
}
