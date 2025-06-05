fn main() {
    println!("Hello, world!");
}

/* 
 
 
 */

fn test(){
   // isi ransel
    let mut isi_ransel = vec![0,1,2,3];
    println!("isi ransel  sebelum : {:?}", isi_ransel)


    // 
    isi_ransel.extend_from_slice(&[4,5,6]);
    println!("isi ransel setelah : {:?}", isi_ransel)
}


/* 
 
 
 */
