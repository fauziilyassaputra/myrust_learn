fn main() {
    println!("Hello, world!");
}

/* 
 
 
 */

#[test]
fn test_iterator() {
    let number_positive = vec![12,-34,-5,7,23];
    let check_positive:   Vec<_> = number_positive.iter().filter(|x| **x > 0).collect();
    println!("{:?}", check_positive);
    // output : [12, 7, 23]

    let some_data = vec!["12","four","9","book", "7", "three"];
    let get_number: Vec<_> = some_data.into_iter().filter_map(|x| x.parse::<i32>().ok()).collect();
    println!("{:?}", get_number);
    // output : [12, 9, 7]
  
}

/* 
- filter mempertahankan elemen yang memenuhi kondisi, serta tidak mengubah elemennya
- filter_map menyarine elemen serta memodifikasi elemen , serta bisa menggunakan Option<T>
*/
