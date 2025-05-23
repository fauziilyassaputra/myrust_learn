

fn main() {
    println!("Hello, world!");
}


#[test]
fn test_string_manipulation() {
    let s = String::from("hoshimi miyabi");

    println!("{}", s.to_uppercase()); // HOSHIMI MIYABI
    println!("{}",s.to_lowercase()); // hoshimi miyabi
    println!("{}", s.len()); //14
    println!("{}", s.replace("miyabi", "harumasa")); // hoshimi harumasa
    println!("{}", s.contains("hoshimi")); //true
    println!("{}", s.starts_with("hoshimi")); //true
    println!("{}", s.ends_with("miyabi")); //miyabi
    println!("{}", s.trim()); //true
    println!("{}", &s[0..7]); //hoshimi
    println!("{:?}", s.get(0..7)); //Some("hoshimi")
}
