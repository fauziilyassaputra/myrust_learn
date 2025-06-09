fn main() {
    println!("Hello, world!");
}

/* 
- cerita :
      sebelum tidur, kita berencana untuk berkemas-kemas agar besok bisa langsung melanjutkan perjalanan . aku akan membereskan
    isi ransel.

- masalah :
      makanan yang ada didalam tas tidak terurut, urutkan!
 
 */

#[test]
fn test2(){
    // isi ranselnya
    let mut isi_ransel = vec!["pisang","apel","daging rusa","pisang","pisang"];
    
    isi_ransel.swap(0,2);
    
    // cek isi ransel setelah durian busuk kita buang
    println!("{:?}", isi_ransel);
    // output : ["daging rusa", "apel", "pisang", "pisang", "pisang"]
    
}
