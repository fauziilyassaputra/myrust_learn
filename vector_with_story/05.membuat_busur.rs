fn main() {
    println!("Hello, world!");
}

/* 
 - cerita :
       malam hari sebentar lagi tiba, kita harus menyiapkan senjata untuk mempertahankan diri. ery berinisiatif untuk membuat busur beserta 
    anak panahnya. 
    setelah berhasil dibuat sekitar 1 sampai 2 jam. kita didatangi oleh 2 monster tulang yang bersiap menyergap kita, ery pun bersiap-siap
    menyerang dengan busur baru yang dibuat , ayo kita bantu dengan mengurusi anak panahnya

- masalah :
  1. buat ukuran wadah anak panah dengan kapasitas 10 anak panah agar bisa menyerang musuh dengan cukup anak panah
  2. isi anak panah yang telah dibuat (jumlah anak panah tersedia ada 8)
  3.cek isi wadah anak panah, serta cek kapasitas wadahnya dengan pasti
 
*/



#[test]
fn mengukur_senjata(){
  // buat wadah anak panahnya
    let mut wadah_anak_panah: Vec<i32> = Vec::with_capacity(10);
  // mengisi wadah dengan anak panah
    for anak_panah in 0..8 {
        anak_panah.push(i);
    }

  // cek jumlah anak panah yang berada di wadah
    println!("{:?}", wadah_anak_panah.len());

  // cek kapasitas wadahnya
    println!("{:?}", wadah_anak_panah.capacity());
}

/*
penjelasan :
    - Vec::with_capacity(10) digunakan untuk mengalokasikan memori awal untuk sebuah vector di rust sebelum elemen benar-benar ditambahkan
    - dengan kita menentukan jumlah elemen yang ditambahkan, akan meningkatkan efesiensi
    - tanpa with_capacity, vector akan akan memperbesar memori secara bertahap yang bisa menyebabkan overhead ekstra
    - kalau kita menggunakan Vector::new() , kapasitas awalnya dalah nol, dan akan meningkat bertahap saat elemen ditambahkan
    
    - with_capacity() jika gagal dalam mengalokasikan memori, maka program akan panic
    - try_with_capacity() tidak menyebabkan panic jika alokasi gagal

    
*/
