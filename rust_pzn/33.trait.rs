fn main() {
    println!("Hello, world!");
}

/*
- trait bisa digunakan untuk tipe data, namun tetap perlu ada implementasi konkritnya, misal menggunakan struct atau enum
*/

struct Person{
    first_name: String,  
    // middle_name: String,
    last_name: String,
    // age: u8
}

trait CanSayHello {
    //default implementation
    fn hello(&self) -> String{
        String::from("hello")
    }

    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}

impl CanSayHello for Person {
    fn say_hello(&self) -> String{
        format!("hello, my name is {}", self.first_name)
    }
    fn say_hello_to(&self, name: &str) -> String{
        format!("Hello {}, my name is {}", name, self.last_name)
    }
}

//  Trait sebagai parameter

fn say_hello_trait(value: &impl CanSayHello){
    println!("{}", value.say_hello())
}

/*
- trait tidak bisa dibuat instancenya
- untuk membuat instance dengan tipe data trait, maka kita harus gunakan implementasinya
*/

#[test]
fn test_trait(){
    let person = Person{
        first_name: String::from("hoshimi"),
        last_name: String::from("miyabi")
    };

    let result = person.say_hello_to("yanagi");
    println!("{}",  result);

    let result = person.hello();
    println!("{}", result);  

    say_hello_trait(&person);
}


trait CanSayGoodBye {
    fn say_goodbye(&self) -> String;
    fn say_goodbye_to(&self, name: &str) -> String;
}

impl CanSayGoodBye for Person{
    fn say_goodbye(&self) -> String{
        format!("goodbye, my name is {}", self.first_name)
    }
    fn say_goodbye_to(&self, name: &str) -> String{
        format!("goodbye {}, my name is {}",name, self.first_name)
    }
}

fn hello_and_goodbye(value: &(impl CanSayHello + CanSayGoodBye)){
    println!("{}", value.say_hello());
    println!("{}", value.say_goodbye());
}

#[test]
fn test_trait_goodbye(){
    let person = Person{
        first_name: String::from("hoshimi"),
        last_name: String::from("miyabi")
    };

    println!("{}", person.say_goodbye());
    println!("{}", person.say_goodbye_to("harumasa"));

    hello_and_goodbye(&person);

    
    //  mencegah conflict method name
    CanSayHello::say_hello(&person);
    Person::say_hello(&person);
}

//  return trait

struct SimplePerson{
    name: String
}

impl CanSayGoodBye for SimplePerson{
    fn say_goodbye(&self) -> String {
        format!("goodbye, my name is {}", self.name)
    }
    fn say_goodbye_to(&self, name: &str) -> String {
        format!("goodbye {}, my name is {}", name, self.name)
    }
}

fn create_person(name: String) -> impl CanSayGoodBye {
    SimplePerson {name}
}

#[test]
fn test_return_trait(){
    let person = create_person(String::from("hoshimi"));
    println!("{}",  person.say_goodbye());
    println!("{}", person.say_goodbye_to("yanagi"));
}


// super trait 
/*
- trait bisa digabungkan dengan konsep mirip pewarisan, dimana satu trait bisa memiliki beberapa trait dibawahnya
- trait yang ada diatasnya bisa kita sebut super trait
*/
