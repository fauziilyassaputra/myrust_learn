fn main() {
    println!("Hello, world!");
}

/* 
 soal :
   bagaimana cara memasukkan string "rust" di dalam variable lesson seperti : "this is a rust lesson"
 
 */


#[test]
fn lets_lesson(){
    // let name = String::from("hoshimi miyabi");
    // let new_name = name.as_str();
    // println!("{}", new_name);

    let mut lesson = String::from("this is a lesson");
    lesson.insert_str(10, "rust ");
    println!("{}",lesson);
    

}


/*
penjelasan :
- sama seperti push_str(), berguna untuk menambahkan teks kedalam sebuah string
- perbedaannya adalah insert_str() bisa meenentukan posisi tertentu, sedangkan push_str() ditambahkan di akhir string
 */
