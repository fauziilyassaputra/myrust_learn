fn main() {
    println!("Hello, world!");
}


/*
&str adalah tipe data text yang fixed size, sehingga disimpan di stack
String adalah tipe  data text yang bisa dikembangkan sehingga disimpan di heap
*/

#[test]
fn my_str(){
    let name: &str = "  hoshimi miyabi  ";
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim); // that's different (space from name not readed)

    // fixed size with mutable
    let mut username: &str = "ellen";  //meskipun sudah diganti, tapi  string 'ellen'  tetap disimpan di memori,  dan tidak terhapus
    username = "lycon"; // hanya mengganti, tidak menghapus string 'ellen' sebelumnya
    println!("{}", username);
}

#[test]
fn my_string(){
    let mut name: String = String::from("hoshimi miyabi");
    println!("{}", name);

    name.push_str(" family");
    println!("{}", name);

    let void = name.replace("family", "void hunter");
    println!("{}", name);
    println!("{}", void);
}
