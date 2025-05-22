fn main() {
    println!("Hello, world!");
}

fn double(value: Option<i32>)  -> Option<i32> {
    match value {
        None => None,
        Some(i) => Some(i * 2)
    }
}

#[test]
fn test_option(){
    let result = double(Some(10));
    println!("{:?}", result);

    let result = double(None);
    println!("{:?}", result)
}
