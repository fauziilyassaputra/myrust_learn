fn main() {
    println!("Hello, world!");
}

/* 
 soal :
 1. terdapat variable vector dari angka 1 sampai 8, kalikan semuanya dengan tiga !
 
 */

#[test]
fn test_iterator() {
    let vector = vec![1,2,3,4,5,6,7,8];
    // gunakan fungsi map lalu isi dengan closure
    let  iter: Vec<_> = vector.iter().map(|x| x * 3).collect();
    println!("{:?}",  iter);
  // output : [3, 6, 9, 12, 15, 18, 21, 24]
}
