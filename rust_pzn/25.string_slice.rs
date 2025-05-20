fn main() {
    println!("Hello, world!");
}

#[test]
fn slice_reference(){
    let name = String::from("Hoshimi miyabi");

    let first_name: &str = &name[0..3];
    println!("{}", first_name);

    let last_name: &str = &name[8..];
    println!("{}", last_name);

}
