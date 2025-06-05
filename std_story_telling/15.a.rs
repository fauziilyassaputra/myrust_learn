fn main() {
    println!("Hello, world!");
}

/* 
 
 
 */


#[test]
fn test(){
    // isi ransel
    let mut isi_ransel = vec![0,1,2,3,3,4,5,6,6,7,2];
    println!("isi ransel  sebelum : {:?}", isi_ransel);


    // 
    isi_ransel.dedup();
    println!("isi ransel setelah : {:?}", isi_ransel)
  //   isi ransel setelah : [0, 1, 2, 3, 4, 5, 6, 7, 2]

}

/* 
 
 
 */
