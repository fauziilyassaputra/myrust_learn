fn main() {
    println!("Hello, world!");
}

/* 
- cerita:
selain daging rusa,  kita juga mendapatkan apel . ayo masukkan ke ransel.

- masalah :
masukkan apel ke dalam ransel dengan apel lainnya
 
 */

#[test]
fn test2(){
    // isi ranselnya
    let mut isi_ransel = vec!["daging rusa","apel","pisang","pisang","pisang"];
    
    isi_ransel.insert(1,"apel");
    
    // cek isi ransel setelah durian busuk kita buang
    println!("{:?}", isi_ransel);
    // output : ["daging rusa", "apel", "apel", "pisang", "pisang", "pisang"]
    
}

/* 



*/
