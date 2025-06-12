fn main() {
    println!("Hello, world!");
}

/* 
 soal :
   coba cek akhir kata pada variable learn, apakah akhir katanya adalah "learn" atau "rust" ?
 
 */


#[test]
fn cek_kata_akhir(){
 
    let learn = "i like rust language, let's learn";
    
    // cek akhir kata pada variable learn
    println!("{}", learn.ends_with("learn"));
    println!("{}", learn.ends_with("rest"));
    
    
    

}


/*


 */
