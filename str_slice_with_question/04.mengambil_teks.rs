fn main() {
    println!("Hello, world!");
}

/* 
 soal :
 bisakah ambil kata "rust" pada variable learn ? dan coba bagaimana kalau kita ambil indeks yang melebihi variablenya ?
 
 */


#[test]
fn ambil_string(){
    // let name = String::from("hoshimi miyabi");
    // let new_name = name.as_str();
    // println!("{}", new_name);
    
    let learn = "i like rust language, let's learn";
    
    // mengambil substring dari indeks 0 sampai 10
    if let Some(substring) = learn.get(7..11){
        println!("subsring : {}", substring)
    } else { println!("indeks tidak valid") }
    
    
    //mencoba mengambil indeks yang tidak valid
    if let Some(substring) = learn.get(7..100){
        println!("subsring : {}", substring)
    } else { println!("indeks tidak valid") }
    
}


/*
jika kita menggunakan teks[7..11] secara langsung dan indeksnya tidak valid akan menyebabkan panic, dengan get(),
kita bisa menangani kasus tersebut dengan aman menggunakan option

 */
