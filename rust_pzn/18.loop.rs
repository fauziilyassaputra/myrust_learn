fn main() {
    println!("Hello, world!");
}


#[test]
fn loop_expression(){
    let mut counter = 0;
    loop{
        counter += 1;

        if counter >= 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("Counter : {}", counter)
    }
}

#[test]
fn loop_return_value(){
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10{
            break counter * 2;
        }
    };
    println!("{}", result);
}

// loop label 
#[test]
fn loop_label(){
    let mut number = 0;
    'outher: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outher;
            }
            println!("{} x {} = {}", number, i, number * i);
            i += 1;
            if i > 20 {
                break;
            }
        }
        number += 1;
    }
}
