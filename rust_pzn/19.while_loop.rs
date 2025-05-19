fn main() {
    println!("Hello, world!");
}


#[test]
fn while_loop(){
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter: {}", counter)
        }
        counter += 1;
    }
}

#[test]
fn while_loop2(){
    let my_array: [&str; 5] = ["A","B","C","D","E"];
    let mut index = 0;

    while index < my_array.len() {
        println!("value: {}", my_array[index]);
        index += 1;
    }
}
