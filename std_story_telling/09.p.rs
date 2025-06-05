fn main() {
    println!("Hello, world!");
}

/* 
- cerita :
  pagi telah tiba, saatnya melanjutkan perjalanan. sebelum berangkat, ery membuatkan busur untukku dan membagi jumlah anak
  panah menjadi dua bagian . ery akan menggunakan anak panah yang genap

- masalah :
    keluarkan anak panah ganjil dari wadahnya !
 
 */

#[test]
fn test2(){
     // isi wadah
    let mut wadah_anak_panah = vec![1,2,3,4,5,6];

    wadah_anak_panah.retain(|&x| x % 2 == 0);

    // cek isi wadah ery setelah di bagikan
    println!("{:?}", wadah_anak_panah)
    // output: [2, 4, 6, 8]
    
}

/*
penjelasan :
  - retain memodifikasi koleksi secara langsung, berbeda dengan filter yang mengembalikan koleksi baru tanpa
  mengubah yang aslinya
*/
