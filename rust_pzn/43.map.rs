


fn main() {
    println!("Hello, world!");
}

/*
- Rust memiliki dua implementasi map, yaitu hashMap dan BTreeMap
- BTreeMap keynya akan diurutkan sedangkan HashMap itu tidak
- maka tentu HashMap lebih cepat dibanding BTreeMap
*/

use std::collections::HashMap;
use std::collections::BTreeMap;


#[test]
fn test_hash_map(){
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("name"), String::from("hoshimi"));
    map.insert(String::from("age"), String::from("21"));

    let name = map.get("name");
    let age = map.get("age");

    println!("Name {}", name.unwrap());
    println!("age {}", age.unwrap());
}

// BTreeMap




#[test]
fn test_btree_map(){
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    map.insert(String::from("name"), String::from("hoshimi"));
    map.insert(String::from("age"), String::from("21"));
    map.insert(String::from("country"), String::from("new eridu"));

    for entry in map {
        println!("{} {}", entry.0, entry.1);
    }
  

}
