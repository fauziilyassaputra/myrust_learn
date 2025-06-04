fn main() {
    println!("Hello, world!");
}

/* 
 - cerita :
     dalam perjalanan kita tentunya memerlukan bekal bukan ?, ayo kita buat dengan kotak yang tersedia . kita
     buat dengan ukuran tidak terlalu besar agar tidak memberatkan . aku punya tiga makanan buat kita bawa .
     
 - masalah :
     tolong buatkan kotak makanannya dengan muatan maksimal 6 item. lalu masukkan 3 makanan ery ke kotak yang 
     barusan di buat!
 */


#[test]
fn test(){
    let mut kotak_makanan: [i32; 6] = [0; 6];
    kotak_makanan[1] = 1;
    kotak_makanan[2] = 2;
    kotak_makanan[3] = 3;
    
    println!("{:?}", kotak_makanan);
    /*
    [1, 2, 3, 0, 0, 0]
    */
}

/* 
- penjelasan :
  ukuran maksimal kotak makanan yang telah di buat adalah 6 item dengan defaultnya adalah 0. jika data mengalami 
  kelebihan muatan maka bakal terjadi error
  
*/
