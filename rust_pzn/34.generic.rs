fn main() {
    println!("Hello, world!");
}

// Generic di struct

// jika ingin buat default tambahkan sama dengan, contoh : <T = i32>
struct Point<T> {
    x: T,
    y: T
}

#[test]
fn test_generatic_struct(){
    let integer = Point::<i32> {
        x:1, y:2
    };
    let float = Point::<f64> {
        x:1.0, y:2.0,
    };

    println!("{} {}", integer.x , integer.y);
    println!("{} {}", float.x , float.y);

}

// Generic di enum

enum Value<T> {
    NONE,
    VALUE(T)
}

#[test]
fn test_generic_enum(){
    let value  = Value::<i32>::VALUE(10);

    match value {
        Value::NONE => {
            println!("none")
        }
        Value::VALUE(value) => {
            println!("value {}", value)
        }
    }
}

// Generic type bound
/*
- saat membuat generic type, kita bisa memberi batasan type yang diperbolehkan
- caranya kita bisa gunakan : (titik dua) diikuti dengan trait
*/

trait CanSayGoodBye {
    fn say_goodbye(&self) -> String;
    fn say_goodbye_to(&self, name: &str) -> String;
}

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


struct HI<T: CanSayGoodBye> {
    value: T
}

#[test]
fn test_generatic_bound(){
    let hi = HI::<SimplePerson> {
        value: SimplePerson {
            name: String::from("hoshimi")
        }
    };
    println!("{}", hi.value.name)
}

// generic di function

fn min<T: PartialOrd>(value1: T, value2: T) -> T {
    if value1 < value2 {
        value1
    } else {
        value2
    }
}

#[test]
fn generic_to_function() {
    let result = min::<i32>(10, 20);
    println!("{}", result);

    let result = min(20.5, 5.3);
    println!("{}", result);
}

// generic di method

// Point berasal dari struct diatas
impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}

#[test]
fn test_generic_method(){
    let point = Point{x:10, y:10};
    println!("{}", point.get_x());
    println!("{}", point.get_y());
    println!("{}", point.get_value());
}

// Generic pada trait
// trait GetValue<T> {
trait GetValue<T> where T: PartialOrd{
    fn get_value(&self) -> &T;
}

// impl<T> GetValue<T> for Point<T>{
impl<T> GetValue<T> for Point<T> where T: PartialOrd{
    fn get_value(&self) -> &T {
        &self.x
    }
}
