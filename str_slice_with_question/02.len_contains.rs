fn main() {
    println!("Hello, world!");
}

/* 
 soal :
 coba cek panjang string pada variable hello, lalu cek apakah ada kata "start" dan "rust" di dalamnya
 
 */


#[test]
fn cek_string(){
   
    let hello = "hello world, let's start learning now";
    
    // panjang str hello :
    println!("{}", hello.len());
    
    // cek kata pada sebuah string slice
    println!("{}", hello.contains("start")); // true
    println!("{}", hello.contains("rust")); // false
    
    

}


/*


 */
