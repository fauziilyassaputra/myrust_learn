fn main() {
    println!("Hello, world!");
}


#[test]
fn for_loop(){
    let my_array: [&str; 5] = ["A","B","C","D","E"];
    

    for array in my_array {
        println!("value array : {}", array)
    }
}

#[test]
fn my_range (){
    let range = 0..5;
    /*
    0 is inclusive
    5 is exclusive
     */

    println!("start: {}", range.start);
    println!("end: {}", range.end);

    let my_array: [&str; 5] = ["A","B","C","D","E"];
    
    for i in range{
        println!("{}", my_array[i]);
    }

    // or :

    // for i in 0..5 {
    //     println!("{}", my_array[i]);
    // }
}

#[test]
fn my_range_inclusive(){
    let range = 0..=4;
    /*
    0 is inclusive
    5 is exclusive
     */

    println!("start: {}", range.start());
    println!("end: {}", range.end());

    let my_array: [&str; 5] = ["A","B","C","D","E"];
    
    for i in range{
        println!("{}", my_array[i]);
    }

}
