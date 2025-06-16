fn main() {
    println!("Hello, world!");
}

/* 
 
 
 */
#[test]
fn test_iterator() {
    let mut data_by_ref = vec!["let's", "play", "with", "rust"].into_iter();
    let check_by_ref: Vec<_> = data_by_ref.by_ref().take(2).collect();
    println!("data yang diambil: {:?}", check_by_ref);
    // data yang diambil: ["let's", "play"]
    
    let data_sisa_by_ref:Vec<_> = data_by_ref.collect();
    println!("data sisa : {:?}", data_sisa_by_ref);
    // data sisa : ["with", "rust"]
}
