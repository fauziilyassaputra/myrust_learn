fn main() {
    println!("Hello, world!");
}

#[test]
fn stack_heap(){
    function_a();
    function_b();  
}

fn function_a(){
    let a = 10;
    let b = String::from("tsukishiro");
    println!("{}, {}", a, b);
}

fn function_b(){
    let a = 10;
    let b = String::from("yanagi");
    println!("{},  {} ", a, b);
}

