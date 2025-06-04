fn main() {
    println!("Hello, world!");
}

/* 
 - cerita :
       pada malam hari kami berburu, selain menemukan pisang, kami menemukan rusa . dengan sigap, ery langsung membidik dan 
     menembak tepat di leher rusanya sehingga membuatnyamati. kami pun memotong badannya dan membakarnya sebagian dan menyimpannya
     sebagian.
     ketika ingin menyimpan daging rusa yang tersisa , ternyata durian yang ada di ransel mengeluarkan bau tidk sedap sehingga
     harus kita keluarkan

 -masalah :
        keluarkan durian busuk yang ada didalam ransel
 */

#[test]
fn membuang_isi_ransel(){
    // isi ranselnya
    let mut isi_ransel = vec!["pisang","apel","durian","pisang","pisang","daging rusa"];
    
    // buang durian busuknya 
    isi_ransel.swap_remove(2);
    
    // cek isi ransel setelah durian busuk kita buang
    println!("{:?}", isi_ransel);
}

/*
- penjelasan :
  - gunakan swap_remove() jika tidak peduli dengan urutan elemen dan ingin performa lebih cepat
  - swap_remove() menghapus elemen dengan menukar elemen terakhir dengan elemen yang dihapus, sedangkan remove() menggeser semua elemen
  setelahnya ke kiri untuk menjaga urutan sehingga lebih lambat daripada swap_remove()
*/
