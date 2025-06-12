fn main() {
    println!("Hello, world!");
}

/* 
 soal :
 apakah awal kata pada variable learn dan learn 2 adalah "rust" ? , coba cek!
 apakah akhir kata pada variable best dan best2 adalah "rust" ? coba cek!
 */


#[test]
fn cek_awal_akhir() {

    let learn = "rust is the best language";
    let best = "i like rust";
    let cek_awal_kata = learn.starts_with("rust");
    let cek_akhir_kata = best.ends_with("rust");
    println!("{}", cek_awal_kata); //true
    println!("{}", cek_akhir_kata);  //true
    
    let learn2 = "php is the best language";
    let best2 = "i like php";
    let cek_awal_kata2 = learn2.starts_with("rust");
    let cek_akhir_kata2 = best2.ends_with("rust");
    println!("{}", cek_awal_kata2); //false
    println!("{}", cek_akhir_kata2);  //false
   
}
/*


 */
