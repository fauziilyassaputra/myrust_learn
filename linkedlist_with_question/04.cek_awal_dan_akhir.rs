fn main() {
    println!("Hello, world!");
}

/* 
 soal :
 1. cek isi awal linked list
 2. cek isi akhir linked list
 
 */
use std::collections::LinkedList;
#[test]
fn test_linked_list() {
    // membuat linked list
    let mut my_linked_list = LinkedList::new();
    
    // mengisi linked list
    my_linked_list.push_back(13);
    my_linked_list.push_back(4);
    my_linked_list.push_back(25);
    my_linked_list.push_back(19);
    my_linked_list.push_back(17);
    
    // cek isi awal dari linked list
    println!("{:?}", my_linked_list.front());
    // output : Some(13)
    
    // cek isi akhir linked list
    println!("{:?}", my_linked_list.back());
    
    
    // cek isi linked list 
    println!("isi linked list : {:?}",my_linked_list);

}
