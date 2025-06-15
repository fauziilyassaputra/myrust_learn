fn main() {
    println!("Hello, world!");
}

/* 
 soal :
 1. coba ambil angka 4 pada variable vector
 2. panggil isi vector [1,4,7,10] !
 
 */

#[test]
fn test_iterator() {
    let vector = vec![1,2,3,4,5,6,7,8,9,10,11,12];
    let iter = vector.iter().nth(3);
    println!("{:?}", iter);
    // output : Some(4)

    let my_step_by: Vec<_> = vector.iter().step_by(3).collect();
    println!("{:?}", my_step_by);
    // output : [1, 4, 7, 10]
}

/* 
- fungsi step_by() pada iterator berfungsi untuk melompati sejumlah elemen dalam setiap  iterasi
*/
