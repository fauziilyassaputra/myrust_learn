fn main (){
  println!("hello world!");
  }

// dekorator / anotation
#[test]
fn my_unit_test(){
  println!("hello test!");
}
// cargo test my_unit_test -- --exact --nocapture  (untuk melihat hasil output dari unit test pada 'my_unit_test')
