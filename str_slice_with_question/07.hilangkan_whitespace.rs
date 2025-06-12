fn main() {
    println!("Hello, world!");
}

/* 
 soal :
   terdapat string dengan whitespace pada variable like, coba hilangkan!
 
 */


#[test]
fn hilangkan_whitespace() {
    // string dengan white space
    let mut like = "i want  \n   learn rust \n \t  language";
    
    // menghilangkan white space
    let clean_like: Vec<&str> = like.split_whitespace().collect();
    println!("{:?}", clean_like);
    // output :  ["i", "want", "learn", "rust", "language"]

}
/*


 */
