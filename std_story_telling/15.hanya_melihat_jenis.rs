fn main() {
    println!("Hello, world!");
}

/* 
 
 
 */


#[test]
fn test(){
    // isi ransel
    let mut isi_ransel = vec!["apel".to_string(),"apel".to_string(),"apel".to_string(),"jeruk".to_string(),"jeruk".to_string(),"pisang".to_string()];
    println!("isi ransel  sebelum : {:?}", isi_ransel);


    //  menghapus data yang sama
    isi_ransel.dedup();
    println!("isi ransel setelah : {:?}", isi_ransel)
  //   isi ransel setelah : ["apel", "jeruk", "pisang"]

}

/* 
 
 
 */
