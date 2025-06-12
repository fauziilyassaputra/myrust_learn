fn main() {
    println!("Hello, world!");
}

/* 
 soal :
   ubah kata "php" menjadi "rust" pada variable like menjadi like_rust !
 
 */


#[test]
fn ganti_teks() {
    // ubah semua kata php dengan rust
    let like = "i like php language, php is the best";
    let like_rust = like.replace("php", "rust");
    println!("{}", like_rust);
    // output :   i like rust language, rust is the best

     // ubah hanya awal kata php menjadi rust
    let info = "php is a good choice, php is elephant language";
    let info_rust = info.replacen("php", "rust", 1);
    println!("{}", info_rust)
    // output : rust is a good choice, php is elephant language
  
}
/*
replace berguna untuk menggantikan substring pada string. replace mengganti semua substring lama dengan yang baru,
berbeda dengan replacen yang hanya menggantikan sejumlah substring sesuai yang di tentukan

 */
