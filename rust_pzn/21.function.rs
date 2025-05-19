fn main() {
    println!("Hello, world!");
}

fn say_hello(){
    println!("Hello");
}

#[test]
fn test_say_hello(){
    say_hello();
    say_hello();
    say_hello();
}

// function with parameter / argument

fn say_goodbye(first_name: &str, last_name: &str){
    println!("Goodbye {} {}", first_name, last_name);
}

#[test]
fn test_say_goodbye(){
    say_goodbye("hoshimi", "miyabi");
    say_goodbye("asaba", "harumasa");
}

// function with return value
fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }
    let mut result = 1;
    for i in 1..=n{
        result *=i;
    }
    result
}

#[test]
fn test_factorial_loop(){

    let result = factorial_loop(5);
    println!("{}", result);

    let result2 = factorial_loop(-10);
    println!("{}", result2);
}


// recursive function
fn recursive_function(value: String, times: u32){
    if times == 0 {
        return;
    } else {
        println!("{}", value)
    }
    recursive_function(value, times - 1);
}

#[test]
fn test_print_recursive(){
    recursive_function(String::from("hoshimi"), 10);
}


fn factorial_recursive(n: u32) -> u32{
    if n <= 1 {
        return 1;
    }
    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursive(){
    let result = factorial_recursive(5);
    println!("{}", result);
}
