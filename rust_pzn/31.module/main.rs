fn main() {
    println!("Hello, world!");
}


mod model {
    struct _User{
        first_name: String,
        last_name: String,
        user_name: String,
        email: String,
        age: u8
    }

    impl _User{
        fn _say_hello(&self, name: &str){
            println!("Hello {}, my name is {}", name , self.first_name);
        }
    }
}

// visibility
/*
- secara default, kode yang ada di dalam module itu sifatnya private, jadi hanya bisa diakses di dalam mdule itu daja
- jika kita ingin mengaksesnya dari luar, kita harus ubah akses dari private menjadi public
*/

mod model_public {
    pub struct User{
        pub first_name: String,
        pub last_name: String,
        pub user_name: String,
        pub email: String,
        pub age: u8
    }

    impl User{
        pub fn say_hello(&self, name: &str){
            println!("Hello {}, my name is {}", name , self.first_name);
        }
    }
}

#[test]
fn test_module(){
    let user = model_public::User{
        first_name: String::from("hoshimi"),
        last_name: String::from("miyabi"),
        user_name: String::from("fox lady"),
        email: String::from("miyabi@hoshimi.ne"),
        age: 21
    };

    user.say_hello("yanagi");
}

// memanggil berbagai macam module (use)

/*
mod first {
    pub fn say_hello(){
        println!("hello from first module")
    }
}
mod second {
    pub fn say_hello(){
        println!("hello from second module")
    }
}
*/

mod first;
mod second;

use first::say_hello;
use second::say_hello as say_hello_second;

#[test]
fn test_use(){
    // first::say_hello();
    // second::say_hello();

    say_hello();
    say_hello_second();

}
