fn main() {
    println!("Hello, world!");
}

/* 
 soal :
    bisakah memisahkan 3 substring pada variable learn, dengan kata "rust language" menjadi satu substring
 
 */


#[test]
fn hilangkan_whitespace() {
    // 
    let learn = "i like rust language";

    // gunakan splitn untuk membaginya menjadi tiga bagian berdasarkan spasi
    let pisah_kata:Vec<&str> = learn.splitn(3, ' ').collect();
    println!("{:?}", pisah_kata);
  //output : ["i", "like", "rust language"]
}

/*
splitn berguna untuk membagi string slice menjadi sejumlah bagian yang ditentukan, berbeda dengan spilt yang
membagi string sepenuhnya.

 */
