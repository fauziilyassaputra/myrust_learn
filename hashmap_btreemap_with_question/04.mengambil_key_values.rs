fn main() {
    println!("Hello, world!");
}

/* 
 
 
 */
use std::collections::HashMap;
// use std::collections::BTreeMap;

#[test]
fn test_linked_list() { 
    // membuat hash map 
    let  my_hashmap: HashMap<i32, &str>  = HashMap::from([
        (1,"hoshimi miyabi"),
        (2,"asaba harumasa"),
        (3,"tsukisiro yanagi"),
        (4,"shokaku")
    ]); 
    
   for (key, value) in my_hashmap.iter(){
        println!("key : {key}, value: {value}")
    }
  /*
  output :
  key : 3, value: tsukisiro yanagi
  key : 1, value: hoshimi miyabi
  key : 2, value: asaba harumasa
  key : 4, value: shokaku
  */
   
}
/*
penjelasan :
   

 */
