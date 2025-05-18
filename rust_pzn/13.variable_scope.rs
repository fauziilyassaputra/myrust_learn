fn main() {
    println!("Hello, world!");
}

#[test]
fn my_variable_scope(){
    let codename = 1; //variable scope

    {  //inner scope
        println!("your code name: {}", codename);
        let your_name = 11;
        println!("your real name: {}", your_name);
    }

    // println!("your real name: {}"  your_name);  // error (variable "your_name" is not from this scope)
}

