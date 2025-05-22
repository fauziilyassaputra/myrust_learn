fn main() {
    println!("Hello, world!");
}

enum  Level{
    Regular,
    Premium,
    Platinum
}

#[test]
fn my_enum(){
    let _level = Level::Premium;

    match _level {
        Level::Regular => {
            println!("Regular")
        }
        Level::Premium => {
            println!("Premium")
        }
        Level::Platinum => {
            println!("Platinum")
        }
    }
}

// enum data

enum Payment{
    CreditCart(String),
    BankTransfer(String, String),
    Ewallet(String, String)
}



// enum method

impl Payment{
    fn  pay(&self,amount: u32){
        match self {
            Payment::CreditCart(number) => {
                println!("paying with credit card {} amount {}", number , amount);
            }
            Payment::BankTransfer(bank,number, ) => {
                println!("paying with bank transfer {} {} amount {}", bank, number, amount);
            }
            //  gunakan underscore jika tidak ingin memanggil salah satu parameter
            Payment::Ewallet(wallet,_, ) => {
                println!("paying with {} amount {}", wallet, amount);
            }
        }
    }
}

#[test]
fn test_payment_with_enum(){
    let _payment1 = Payment::CreditCart(String::from("123123"));
    _payment1.pay(20000);

    let _payment2 =  Payment::BankTransfer(String::from("BCA"), String::from("123123"));
    _payment2.pay(320000);

    let _payment3 = Payment::Ewallet(String::from("dana"), String::from("123123"));
    _payment3.pay(380000);
}

//  pattern maching pada value

#[test]
fn test_match_value(){
    let name = "wise";
    let name2 = "yixuan";

    match name{
        "ceasar" => {
            println!("welcome chief ");
        }
        "lighter" => {
            println!("welcome lighter");
        }
        other => {
            println!("welcome {}", other);
        }
    }

    // multiple pattern ( | )
    match name2 {
        "hoshimi" | "yixuan" | "hugo" => {
            println!("welcome bos");
        }
        _ => {
            println!("welcome guess")
        }
    }
}

// Range Pattern
/*
- cocok jika datanya sangat banyak yang mana sangat menyulitkan jika menggunakan multiple pattern
- saat ini, range yang digunakan adalah tipe data inclusive range
*/

#[test]
fn test_range_patterns(){
    let value = 67;

    match value{
        75..=100 => {
            println!("Great");
        }
        50..=74 => {
            println!("good");
        }
        25..=49 => {
            println!("bad");
        }
        0..=24 => {
            println!("so bad")
        }
        other => {
            println!("invalid value {}", other);
        }
    }
}

// destructuring struct patterns

struct Geopoint(f64, f64);

impl Geopoint{
    fn new(long: f64,lat: f64) -> Geopoint{
        Geopoint(long, lat)
    }
}

struct Person{
    first_name: String,  
    middle_name: String,
    last_name: String,
    age: u8
}

#[test]
fn test_struct_patterns(){
    let point = Geopoint::new(2.0, 1.0);

    match point {
        Geopoint(long,0.0) => {
            println!("long: {}", long);
        }
        Geopoint(0.0, lat) => {
            println!("lat : {}", lat);
        }
        Geopoint(long,lat) => {
            println!("long: {}, lat: {}", long, lat);
        }
    }

    let person: Person = Person{
        age: 21,
        first_name: String::from("alex"),
        middle_name: String::from("andrina"),
        last_name: String::from("sebastane")
    };

    match person {
        Person{first_name, last_name, ..} => {
            println!("welcome {} {}", first_name, last_name);
        }
    }
}


//  Ignoring

#[test]
fn ignoring_range(){
    let value = 67;

    match value{
        75..=100 => {
            println!("Great");
        }
        50..=74 => {
            println!("good");
        }
        25..=49 => {
            println!("bad");
        }
        0..=24 => {
            println!("so bad")
        }
        // gunakan _ untuk ignoring
        _ => {
            println!("invalid value ");
        }
    }
}

// match expression

#[test]
fn test_match_expression(){
    let value = 2;

    let result = match value {
        1 => {
            "satu"
        },
        2 => {
            "dua"
        }
        _ =>"invalid"
    };
    println!("{}", result);
}
