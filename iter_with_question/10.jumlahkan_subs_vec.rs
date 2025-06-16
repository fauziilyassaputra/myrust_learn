fn main() {
    println!("Hello, world!");
}

/* 
 soal :
 1.jumlahkan semua isi varible angka
 
 */


#[test]
fn test_iterator() {
    let angka = vec![1,2,3,4,5];
    let sum = angka.iter().fold(0, |acc, &x| acc + x);
    println!("{:?}", sum);
     // output : 15

    let sum2: i32= angka.iter().sum();
    println!("{:?}", sum2);
    // output : 15
}
/*
penjelasan :
   

 */

