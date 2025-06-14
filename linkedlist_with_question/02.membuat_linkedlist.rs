fn main() {
    println!("Hello, world!");
}

/* 
 soal :
 1. buatlah linkedlisr kosong
 2. isi linkedlist dari belakang dengan tiga angka terurut
 3. cek panjang linkedlist setelah diisi
 4. hapus isi linked list
 5. cek apakah linked list sudah kosong 
 
 */
use std::collections::LinkedList;

fn create_linked_list() {
  // membuat linked list
    let mut my_linked_list = LinkedList::new();
    
    // mengisi linked list
    my_linked_list.push_back(1);
    my_linked_list.push_back(2);
    my_linked_list.push_back(3);

    
    // melihat isi linked list
    println!("{:?}",my_linked_list);
    // output : [1, 2, 3]
    
    // cek panjang linked list
    println!("panjang linked list : {:?}", my_linked_list.len());
    // panjang linked list : 3

    // menghapus isi linked list
    my_linked_list.clear();

    // cek apakah linked list kosong 
    println!("{:?}", my_linked_list.is_empty());
    // output : true

}
