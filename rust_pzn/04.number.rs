fn main() {
    println!("Hello, world!");
}


#[test]
fn my_number(){
    // default number
    // integer : i32
    // float : f64

    let a = 10;
    println!("{}", a);
    let b = 25.5;
    println!("{}", b);

    let c: i8 = 5;
    println!("{}", c);
    let d: f32 = 16.7;
    println!("{}", d);
}

#[test]
fn  my_number_conversion(){
    let a:i8 = 5;
    println!("{}", a);

    let b:i16 = a as i16;
    println!("{}", b);

    let c:i32 = b as i32;
    println!("{}", c);

    // integer overflow example 
    let d:i64 = 100000;
    let e:i8 = d as i8;
    println!("{}", e);

}
