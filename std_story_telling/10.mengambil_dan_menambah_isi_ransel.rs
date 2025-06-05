fn main() {
    println!("Hello, world!");
}

/* 
 - cerita :
       dalam perjalanan, kami mengonsumsi bagian paling atas(paling akhir), yaitu sebuah pisang. lalu kami menulusuri sungai kecil 
     dan rencana bermalam di sekitar sungai tersebut. aku menyiapkan anak panah untuk melawan musuh selawan musuh yang akan
     dihadapi selanjutnya. sedangkan ery pergi memancing ikan dan mendapatkan 3 ikan kecil untuk disantap pada malam hari. 
     sementara ikan disimpan di ransel, kita juga mencari ranting-ranting kering untuk api unggun .

- masalah :
  1. cek keadaan ransel setelah pisang dikonsumsi
  2. cek keadaan ransel setelah ikan dimasukkan kedalam ransel
  3.cek keadaan ransel setelah ikan dikeluarkan untuk dibakar
     
 
 */

#[test]
fn test2(){
    // isi ransel
    let mut isi_ransel = vec!["daging rusa","apel","pisang","pisang","pisang"];
    println!("isi ransel: {:?}", isi_ransel);

    // mengambil pisang
    isi_ransel.pop();
    println!("isi ransel setelah mengambil pisang: {:?}", isi_ransel);
    
    //memasukkan 3 ikan kedalam ransel
    let mut ikan_ikan = vec!["ikan","ikan","ikan"];
    isi_ransel.append(&mut ikan_ikan);
    println!("isi ransel setelah memasukkan ikan: {:?}", isi_ransel);
    
    //mengambil tiga ikan
    let ambil_ikan: Vec<_> = isi_ransel.drain(4..).collect();
    println!("isi ransel setelah mengeluarkan ikan: {:?}", isi_ransel);

    
}
