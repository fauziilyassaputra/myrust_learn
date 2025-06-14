fn main() {
    println!("Hello, world!");
}

/* 
 soal :
 1.buatlah hashmap kosong dengan value str dengan key integer!
 2.isi dengan  tiga elemen
 3.cek isi hashmap yang telah dibuat

 4.buat hashmap kosong dengan kapasitas 10
 5.cek apakah hashmap dengan kapasitas 10 itu kosong ?
 
 */
use std::collections::HashMap;

#[test]
fn test_linked_list() { 
    // membuat hashmap dengan new
    let mut my_hashmap: HashMap<&str, i32>  = HashMap::new(); 
    
    // mengisi hashmap kosong
    my_hashmap.insert("cat",1);
    my_hashmap.insert("fox",2);
    my_hashmap.insert("tiger",3);
    
    // cek isi my_hashmap
    println!("{:?}",my_hashmap);
    // output : {"tiger": 3, "cat": 1, "fox": 2}
    
    // membuat hashmap dengan capacity
    let you_hashmap: HashMap<String, i32> = HashMap::with_capacity(10);
    
    // cek apakah you_hashmap kosong
    println!("{:?}", you_hashmap.is_empty());
}
/*
penjelasan :
    output dari  variable my_hashmap selalu berubah-ubah posisinya, jika ingin berurutan
    gunakan BTreeMap

 */
