fn main() {
    println!("Hello, world!");
}

#[test]
fn test_variable(){
    //  let is imutable, we can't change it
    let name = "Hoshimi miyabi";
    println!("welcome {}", name);
}

#[test]
fn test_mutable(){
    // let mut is mutable, we can change it
    let mut partner_name = "asaba harumasa";
    println!("welcome {}", partner_name);

    partner_name = "yanagi";
    println!("hello {}", partner_name);
}

// rust is static typing language, we can't replace variable string to variable number   
#[test]
fn test_static_typing(){
    let name = "shokaku";
    println!("welcome back  {}", name);

    // must be error
    // name = 10;
    println!("welcome back  {}", name);
    
}

// Shadowing
// we can make some variable at the same name but the first variable will be closed or we call it 'shadowing'
#[test]
fn test_shadowing(){
    let robot_name = "sharkboo";
    println!("welcome back  {}", robot_name);

    let robot_name = "magnetboo";
    println!("welcome back  {}", robot_name);
    
}
