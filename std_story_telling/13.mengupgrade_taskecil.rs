fn main() {
    println!("Hello, world!");
}

/* 
 - cerita :
 sebelum berangkat, ery akan mengupgrade tas kecilnya agar muat sampai 5 item.
 = masalah :
 ayo upgrade tas sampai muat 5 item 
 
 
 */

#[test]
fn test2(){
    // isi ransel kecil 
    let mut isi_ransel = vec!["ikan".to_string(),"ikan".to_string(),"ikan".to_string()];
    println!("isi ransel: {:?}", isi_ransel);

    // cek total isi ransel
    isi_ransel.resize_with(5, || "kosong".to_string());
    println!(" isi ransel kecil setelah di upgrade: {:?}", isi_ransel);
    //  isi ransel kecil setelah di upgrade: ["ikan", "ikan", "ikan", "kosong", "kosong"]
    
}

/*
penjelasan:
- resize_with() berguna ketika ingin mengubah ukuran vec dengan cara menambahkan elemen baru menggunakan sebuah fungsi
- resize_with() menambahkan elemen baru yang dihasilkan oleh fungsi sehingga lebih fleksibel , berbeda dengan resize() yang
menambahkan elemen baru dengan nilai tetap
*/
