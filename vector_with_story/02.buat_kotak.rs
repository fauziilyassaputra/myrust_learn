fn main() {
    println!("Hello, world!");
}

/* 
 - cerita :
     dalam perjalanan kita tentunya memerlukan bekal bukan ?, ayo kita buat dengan kotak yang tersedia . kita
     buat dengan ukuran tidak terlalu besar agar tidak memberatkan . aku punya tiga makanan buat kita bawa .
     ada pisang, apel, dan durian.
     
 - masalah :
     tolong buatkan kotak makanannya dengan muatan maksimal 6 item. lalu masukkan 3 makanan ery ke kotak yang 
     barusan di buat!
 */


#[test]
fn test(){
    let mut kotak_makanan: [&str; 6] = ["_"; 6];
    kotak_makanan[0] = "pisang";
    kotak_makanan[1] = "apel";
    kotak_makanan[2] = "durian";

    println!("{:?}", kotak_makanan)
    /*
    ["pisang", "apel", "durian", "_", "_", "_"]
    */
}

/* 
- penjelasan :
  ukuran maksimal kotak makanan yang telah di buat adalah 6 item dengan defaultnya adalah "_". jika data mengalami 
  kelebihan muatan maka bakal terjadi error
  
*/
