fn main() {
    println!("Hello, world!");
}

fn print_number(number: i32){
    println!("number {}", number);
}
fn hi(name: String){
    println!("name {}", name);
}

#[test]
fn test_hi(){
    let number = 10;
    print_number(number);  // print_number(10)
    println!("{}", number);

    let name = String::from("hoshimi");
    hi(name);
    // println!("{}", name); //error, because value borrewed here after removed

}

//  Return value ownership

fn full_name(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_fullname(){
    let first_name = String::from("hoshimi");
    let last_name = String::from("miyabi");

    let name = full_name(first_name, last_name);
    println!("{}", name);
    // println!("{}", first_name); // error
    // println!("{}", last_name); // error
}

// return ownership

fn return_full_name(first_name: String, last_name: String) -> (String,String,String) {
    let full_name = format!("{} {}", first_name, last_name);

    (first_name,last_name, full_name)
}
// lebih baik menggunakan reference karena akan menyulitkan jika parameternya terlalu banyak

#[test]
fn return_test_fullname(){
    let first_name = String::from("hoshimi");
    let last_name = String::from("miyabi");

    let (first_name, last_name, name) = return_full_name(first_name, last_name);
    println!("{}", name);
    println!("{}", first_name); //not error
    println!("{}", last_name); //not error
}
