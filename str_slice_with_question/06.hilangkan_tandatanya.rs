fn main() {
    println!("Hello, world!");
}

/* 
 soal:
  hilangkan tanda tanya pada string di variable ada_tanda_tanya
 
 */


#[test]
fn hilangkan_tandatanya() {
    // 
    let ada_tandatanya = "let's?learn?rust?now!?,?before?late";

    // hilangkan ? menggunakan split
    let hilangkan_tandatanya:Vec<&str> = ada_tandatanya.split("?").collect();
    println!("{:?}", hilangkan_tandatanya);
    // output :  ["let's", "learn", "rust", "now!", ",", "before", "late"]
}

/*


 */
