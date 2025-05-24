fn main() {
    println!("Hello, world!");
}

#[test]
fn test_dereference(){
    let value1 = Box::new(10);
    let value2 = Box::new(20);

    // gunakan * agar bisa mengambil nilai aslinya di box

    let result = *value1 * *value2;
    println!("{}", result);
}

// deref trait

use std::ops::Deref;

struct MyValue<T> {
    value: T
}

impl<T> Deref for MyValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
    &self.value
    }
}

#[test]
fn test_dereference_struct(){
    let value = MyValue {value :10};
    let real_value = *value;
    println!("value {}", real_value);
}

fn say_hello_reference(name: &String){
    println!("hello {}", name)
}

#[test]
fn test_deref_reference(){
    let nama = MyValue {
        value: "Hoshimi".to_string()
    };
    say_hello_reference(&nama);
}

