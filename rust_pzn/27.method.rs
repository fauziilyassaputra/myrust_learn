fn main() {
    println!("Hello, world!");
}

struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8
}

impl Person {
    fn say_hello(&self, name: &str){
        println!("Hello {}, my name is {}", name, self.first_name)
    }
}

#[test]
fn test_method(){
    let first_name = String::from("alex");
    let middle_name = String::from("andrina");
    let last_name = String::from("sebastane");
    let age = 21;


    let person = Person{
        first_name,
        middle_name,
        last_name,
        age
    };

    person.say_hello("rina");
    println!("{}", person.first_name); // tidak error karena methodnya menggunakan parameter reference(&)

}

// associated function

struct Geopoint(f64, f64);

impl Geopoint{
    fn new(long: f64,lat: f64) -> Geopoint{
        Geopoint(long, lat)
    }
}
#[test]
fn test_associated_function(){
    let geo_point  = Geopoint::new(10.0, 10.0);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}
