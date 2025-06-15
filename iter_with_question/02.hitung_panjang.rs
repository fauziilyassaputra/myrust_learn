fn main() {
    println!("Hello, world!");
}

/* 
 soal:
 1. hitung panjang vector selain menggunakan len
 2. filter angka ganjilnya
 
 */

#[test]
fn test_iterator() {
    let vector = vec![1,2,2,1,4,4,7];
    let hitung_vector = vector.iter().count();
    println!("panjang vector : {}", hitung_vector);
    // panjang vector : 7

    let genap = vector.iter().filter(|&&x| x % 2 == 0).count();
    println!("jumlah angka genap: {}", genap);
    // jumlah angka genap: 4
    
    

}
/*
penjelasan :
gunakan count() jika bekerja dengan iterator yang tidak memiliki ukuran tetap seperti hasil .filter() dan map()

 */
