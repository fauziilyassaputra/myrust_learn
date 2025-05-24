fn main() {
    println!("Hello, world!");
}


#[test]
fn test_iterator(){
    let array = [1,2,3,4,5];
    let mut iterator = array.iter();

    while let Some(value) = iterator.next(){
        println!("{}", value);
    }

    for value in iterator{
        println!("{}", value);
    }
}

/*
- Iterator memiliki banyak sekali method yang bisa kita gunakan untuk memanipulasi data
- kebanyakan method iterator menggunakan Closure sebagai parameternya
*/

// use std::collections::Vectors;

#[test]
fn test_iterator_method(){
    let vector = vec![1,2,3,4,5,6,7,8,9,10];
    println!("{:?}", vector);

    let sum: i32 = vector.iter().sum(); // menghitung jumlah nilai pada vector yang terdaoat di masing- masing data
    println!("{}",sum);

    let count = vector.iter().count(); // menghitung jumlah data pada vektor
    println!("{}", count);

    let doubled: Vec<i32> =  vector.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);

    let odd: Vec<&i32> = vector.iter().filter(|x| *x % 2 != 0).collect();
    println!("{:?}", odd);
}
