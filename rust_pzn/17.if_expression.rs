fn main() {
    println!("Hello, world!");
}


#[test]
fn if_expression(){
    let value = 7;

    if value >= 8 {
        println!("Good");
    } else {
        println!("not good");
    }
    // return "not good"
}

#[test]
fn else_if_expression(){
    let value = 7;

    if value >= 8 {
        println!("Good");
    } else if value >= 6 {
        println!("not bad");
    } else if value >= 3 {
        println!("bad")
    }
    else {
        println!("very bad");
    }
    // return "not bad"
}

// let statement (manual)
#[test]
fn manual_let_statement(){
    let value = 5;
    let result: &str;

    if value >= 8 {
        result = "Good";
    } else if value >= 6 {
        result ="not bad";
    } else if value >= 3 {
        result ="bad"
    }
    else {
        result ="very bad";
    }
    // return "not bad"
    println!("{}", result);
}

// let statement (declaration)
#[test]
fn let_statement(){
    let value = 5;

    let result =if value >= 8 {
          "Good"
    } else if value >= 6 {
         "not bad"
    } else if value >= 3 {
         "bad"
    }
    else {
         "very bad"
    };
    // return "not bad"
    println!("{}", result);
}

