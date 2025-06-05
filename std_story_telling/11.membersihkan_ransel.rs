fn main() {
    println!("Hello, world!");
}

/* 
 cerita :
       malam pun tiba,kami pun menghabiskan ikan yang telah kami tangkap sebelumnya. setelah makan ikan, kami juga makan apel
     yang tersedia di ransel. lalu meletakkan ransel tersebut dengan keadaan terbuka.
     saking asiknya dengan makan malam, tanpa sadar ransel kita dikerumuni monyet yang ingin pisang kami. kamipun ingin 
     berusaha merebut hingga menyebabkan buah pisangnya hancur mengotori ranselnya

masalah :
      1.bersihkan ransel dengan cara mengkosongkan isinya
      2.cek apakah ransel benar-benar kosong
      
 */

#[test]
fn test2(){
    // isi ransel
    let mut isi_ransel = vec!["pisang hancur", "pisang hancur"];
    println!("isi ransel: {:?}", isi_ransel);
    //  isi ransel: ["pisang hancur", "pisang hancur"]

    // membuang pisang yang hancur
    isi_ransel.clear();
    println!("membersihkan isi ransel selesai, isi ransel: {:?}", isi_ransel);
    // membersihkan isi ransel selesai, isi ransel: []
  
    
    //cek apakah ransel benar-benar sudah kosong
    let cek_isi_ransel = isi_ransel.is_empty();
    println!("apakah ranselnya sudah kosong : {}", cek_isi_ransel);
    // apakah ranselnya sudah kosong : true

}
