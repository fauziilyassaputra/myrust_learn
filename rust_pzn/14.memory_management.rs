fn main() {
    println!("Hello, world!");
}

/* 
- Rust tidak menggunakan garbagge collection seperti java, dan tidak mengalokasikan memori secacra manual seperci C, tapi rust membagi data di dalam memori menjadi dua,
yaitu stack dan heap.
- stack adalah bagian dimana data disimpan dalam struktur data tumpukan, last in first out. dan semua data di stack harus yang fixed size (ukuran data sudah pasti)
-heap adalah tempat untuk menyimpan data yang mana terdapat pointer(petunjuk) ke lokasi dimana data itu berada di heap
-pointer dari heap harus berukuran fixed, oleh karena itu pointer akan disimpan di stack
*/

#[test]
fn stack_heap(){
    function_a();
    function_b();  
}

fn function_a(){
    let a = 10;
    let b = String::from("tsukishiro");
    println!("{}, {}", a, b);
}

fn function_b(){
    let a = 10;
    let b = String::from("yanagi");
    println!("{},  {} ", a, b);
}

/* 
-function_a dan function_b berada didalam stack
-variable a pada kedua function berada pada stack
-variable b pada kedua function adalah string, maka diletakkan kedalam heap
-function_b terletak di atas function_a dan yang pertama kali dihapus adalah function_b
*/






