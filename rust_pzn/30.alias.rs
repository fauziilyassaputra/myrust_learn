fn main() {
    println!("Hello, world!");
}

type Age = u8;
type IdentifyNumber = String;

struct Customer {
    id: IdentifyNumber,
    name: String,
    age: Age
}

#[test]
fn test_customer(){
    let customer = Customer{
        id : String::from("2134"),
        name: String::from("hoshimi"),
        age: 20
    };

    println!("{} {} {}", customer.id, customer.name, customer.age)
}
