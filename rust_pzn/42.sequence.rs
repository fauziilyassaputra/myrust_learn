


fn main() {
    println!("Hello, world!");
}


/*
sequence memiliki beberapa tipe data yaitu :
1. Vec (vector)
2. VecDeque
3. LinkedList
*/

/*
- vector merupakan sequnece yang urutannya sesuai dengan yang kita inginkan
- menambahkan dat ke vector dilakukan dibagian belakang
-cocok untuk implementasi stack(tumpukan), Last In first Out
*/

#[test]
fn test(){
    let mut names = Vec::<String>::new();
    names.push(String::from("vivian"));
    names.push(String::from("banshee"));
    
    for name  in &names{
        println!("{}", name);
    };

    println!("{:?}", names);
}


// VecDeque

/*
- kemampuan manambah data di depan(head) dan dibelakang (end)
- vecDeque cocok digunakan untuk implementasi Queue (Antrian), fiorst in first out
*/

use std::collections::VecDeque;
use std::collections::LinkedList;


#[test]
fn test_vector_deque(){
    let mut names = VecDeque::<String>::new();
    names.push_back(String::from("hoshimi"));
    names.push_back(String::from("miyabi"));
    names.push_front(String::from("family"));

    for name in &names {
        println!("{}", name);
    }

    println!("{}", names[1]); 

}

// linked list
#[test]
fn test_linked_list(){
    let mut names = LinkedList::<String>::new();
    names.push_back(String::from("hoshimi"));
    names.push_back(String::from("miyabi"));
    names.push_front(String::from("family"));

    for name in &names {
        println!("{}", name);
    }

    // println!("{}", names[1]); //error
}
