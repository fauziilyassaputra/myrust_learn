fn main() {
    println!("Hello, world!");
}

/* 
 
 
 */
use std::collections::HashMap;
// use std::collections::BTreeMap;

#[test]
fn test_hashmap() { 
    // membuat hashmap 
    let  my_hashmap: HashMap<i32, &str>  = HashMap::from([
        (1,"hoshimi miyabi"),
        (2,"asaba harumasa"),
        (3,"tsukisiro yanagi"),
        (4,"shokaku")
    ]); 
    
    // cek semua isi value  pada  variable my_hashmap
    for val in my_hashmap.values() {
        println!("{val}")
    }
    
    // cek semua isi keys pada variable my_hashmap
    for key in my_hashmap.keys() {
        println!("{key}")
    }
   
}
/*
penjelasan :
   

 */
