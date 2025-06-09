fn main() {
    println!("Hello, world!");
}

/* 
 soal :
bagaimana cara mengubah string menjadi string slice lalu mengubahnya menjadi uppercase ?
 
 */


#[test]
fn test(){
    let name = String::from("hoshimi miyabi");
    let new_name = as_str(name);
    println!("{}", new_name);
  
    let mut hello = String::from("hello world");
    let uppercae_hello = hello.as_mut_str();
    uppercae_hello.make_ascii_uppercase();
    println!("{}",uppercae_hello);
  
    // output :  HELLO WORLD

}

/*  
penjelasan:
 as_str() berguna untuk mengambil string slice dari string tanpa perlu mengalokasi ulang, sehingga lebih efisien.

*/
