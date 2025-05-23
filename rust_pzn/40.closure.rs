

fn main() {
    println!("Hello, world!");
}


#[test]
fn test_closure(){
    let sum: fn(i32,i32) -> i32 = |value1, value2|  -> i32 {
        value1 + value2
    };

    let result = sum(10,10);
    println!("{}", result);
}

// Closure sebagai parameter

fn print_with_filter(value: String, filter: fn(String) -> String){
    let result = filter(value);
    println!("{}", result);
}

#[test]
fn test_closure_as_parameter(){
    let filter = |value: String| -> String {
        value.to_uppercase()
    };
    print_with_filter(String::from("hoshimi"), filter);
}

// Closure dari function

fn to_uppercase(value: String) -> String{
    value.to_uppercase()
}
#[test]
fn test_function_as_closure(){
    print_with_filter(String::from("hoshimi"), to_uppercase);
}

// closure scope (tidak disarankan)
/*
-karena closure yang merubah data variable terletak di scope bawahnya yang mana biasanya perubahan itu
terjadi dari atas ke bawah dan bukan sebaliknya sehingga membuat bingung jika didalam projek besar
- bungkus dengan struct sebagai solusinya
*/

#[test]
fn test_closure_scope(){
    let mut counter = 0;

    let mut increment = || {
        counter += 1;
        println!("Increment");
    };

    increment();
    increment();
    increment();

    println!("Counter {}", counter);
}

struct Counter {
    counter: i32
}

impl Counter {
    fn increment(&mut self){
        self.counter += 1;
        println!("increment")
    }
}

#[test]
fn test_counter(){
    let mut counter = Counter{counter: 0};
    counter.increment();
    counter.increment();
    counter.increment();

    println!("counter {}" , counter.counter)
}
