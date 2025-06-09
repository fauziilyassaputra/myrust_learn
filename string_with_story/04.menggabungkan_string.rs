fn main() {
    println!("Hello, world!");
}

/* 
 soal :
 bagaimana cara menambah string "world" pada variable hello tanpa membuat variable baru ?
 
 */


#[test]
fn gabung_string(){
    let mut hello = String::from("hello");
    hello.push_str(" world");
    println!("{}",hello);
    // output : hello world

}

/* 
penjelasan :
push_str() adalah cara efektif untuk memperluas string dengan string slice tanpa perlu alokasi baru
*/
