fn main() {
    println!("Hello, world!");
}

/* 
 soal :
 ada string "hello world", bisakah kita lihat bytes tiap hurufnya ?
 
 */


#[test]
fn melihat_bytes(){
    let mut hello = String::from("hello world");
    let hello_bytes = hello.as_bytes();
    println!("{:?}", hello_bytes);
    //  output : [104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]

}

/*
kapan as_bytes() dibutuhkan :
  -saat mengirim data melalui jaringan dalam format biner
  -saat menyimpan string ke file dalam bentuk byte
  -saat menggunakan hashing atau encoding seperti UTF-8


*/
