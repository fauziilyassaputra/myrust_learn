fn main() {
    println!("Hello, world!");
}

/* 
 soal :
 1. ambil key dan value dari variable my_hahmap
 2. hitung panjang len
 3. ambil salah satu data dari varibale my_hashmap
 
 */
use std::collections::HashMap;
// use std::collections::BTreeMap;

#[test]
fn test_hashmap() { 
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

    // menghitung panjang hashmap
    println!("{:?}",my_hashmap.len());
    
    // mengambil hashmap ketiga
    let data_ketiga =  my_hashmap.get(&3);
    println!("data ketiga adalah : {:?}", data_ketiga);

}
/*
penjelasan :
   

 */
