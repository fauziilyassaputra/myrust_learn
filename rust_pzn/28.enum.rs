fn main() {
    println!("Hello, world!");
}

enum  level{
    Regular,
    Premium,
    Platinum
}

#[test]
fn my_enum(){
    let _level = level::Premium;
}

// enum data

enum Payment{
    CreditCart(String),
    BankTransfer(String, String),
    Ewallet(String, String)
}

#[test]
fn test_payment(){
    let _payment1 = Payment::CreditCart(String::from("123123"));
    let _payment2 =  Payment::BankTransfer(String::from("BCA"), String::from("123123"));
    let _payment3 = Payment::Ewallet(String::from("dana"), String::from("123123"));
}

// enum method

impl Payment{
    fn  pay(&self,amount: u32){
        println!("paying amount {}", amount);
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
