fn main() {
    println!("Hello, world!");
}


#[test]
fn numeric_operator(){
    let a = 10;
    let b = 10;
    let c = a * b;
    println!("hasil kali :{}", c);
    let d = a / b;
    println!("hasil bagi :{}", d);
    let e = a + b;
    println!("hasil tambah :{}", e);
}

// augmented assigments
#[test]
fn augmented_assignments(){
    // variable must be mutable
    let mut a = 25;
    println!("variable : {}", a);
    a += 5;
    println!("hasil tambah :{}", a);
    a -= 10;
    println!("hasil kurang :{}", a);

}

