fn main() {
    println!("Hello, world!");
}

/* 
 -  cerita :
       saat melanjutkan perjalanan, tidak lama kemudian ery menemukan sebuah ransel. dia berkata " ayo kita ganti
       kotak kita dengan ransel. agar lebih fleksibel".

- masalah :
      ambillah ransel yang telah ditemukan . lalu kita masukkan dengan makanan yang telah kita punya didalam kotak
 
*/


#[test]
fn test(){

  // ayo kita ambil ranselnya
   let mut ransel = Vec::new();

  // masukkan makanan kita ke ransel kosong yang telah kita ambil
    ransel.push(String::from("pisang"));
    ransel.push(String::from("apel"));
    ransel.push(String::from("durian"));
    
  // cek isi ransel
    println!("{:?}", ransel);

  /*
   output:
    ["pisang", "apel", "durian"]
   */
}



/*

- penjelasan :
   -  kita ubah dari array biasa menjadi vector agar lebih fleksibel, bisa menambahkan atau menghapus elemen. 
      kekurangannya jauh lebih lambat dikarenakan disimpan di heap
   -  
*/

