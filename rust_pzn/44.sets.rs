fn main() {
    println!("Hello, world!");
}


/*
- HashSet tidak menjamin urutan data, karena tujuan hashset memastikan tidak ada data duplikat secara cepat
- BTeeSet selain memastikan tidak ada data duplikat, juga mengurutkan data di dalam setnya , maka performanya lebih lambat dari hashset

*/

use::std::collections::HashSet;
use::std::collections::BTreeSet;


#[test]
fn test_hash_set(){
    let mut set: HashSet<String> = HashSet::new();
    set.insert(String::from("hoshimi"));
    set.insert(String::from("hoshimi"));
    set.insert(String::from("miyabi"));
    set.insert(String::from("miyabi"));
    set.insert(String::from("family"));
    set.insert(String::from("family"));

    for value in set {
        println!("{}", value)
    }
}

#[test]
fn test_btree_set(){
    let mut set: BTreeSet<String> = BTreeSet::new();
    set.insert(String::from("hoshimi"));
    set.insert(String::from("hoshimi"));
    set.insert(String::from("miyabi"));
    set.insert(String::from("miyabi"));
    set.insert(String::from("family"));
    set.insert(String::from("family"));

    for value in set {
        println!("{}", value)
    }
}
