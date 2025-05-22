mod first;
mod second;
mod third;

fn main() {
    println!("Hello, world!");
}


use first::say_hello;
use second::say_hello as say_hello_second;

#[test]
fn test_use(){
    // first::say_hello();
    // second::say_hello();

    say_hello();
    say_hello_second();
    first::second::third::say_hello();

}
