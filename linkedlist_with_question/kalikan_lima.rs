fn main() {
    println!("Hello, world!");
}

/* 
 soal :
 1. buatlah linkedlist kosong
 2. tambahkan beberapa linked list dengan angka integer
 3. kalikan semua isi linked list dengan 5
 
 */
use std::collections::LinkedList;

#[test]
fn test_linked_list() {
    // membuat linked list
    let mut my_linked_list = LinkedList::new();
    
    // mengisi linked list
    my_linked_list.push_back(1);
    my_linked_list.push_back(2);
    my_linked_list.push_back(3);

    // pecah isi linkedlist lalu kalikan 5
     for isi in my_linked_list.iter_mut() {
        *isi *= 5;
    }
    
    // cek isi linked list terbaru
    println!("isi linked list setelah dikali 5: {:?}",my_linked_list);
    // isi linked list setelah dikali 5: [5, 10, 15]
}

/*


 */
