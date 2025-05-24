fn main() {
    println!("Hello, world!");
}

/*
- value hampir sama seperti constan
- value di constant tidak bisa diubah lagi, sedangkan pada static, kita bisa buat mutable static, yang valuenya bisa diubah lagi
- namun karena static itu bisa diakses oleh siapapun, jadi ada kemungkinan tidak aman, misal terjadi race condition
- untuk mengubah mutable static , kita wajib menggunakan unsafe block, atau unsafe function
*/

static mut COUNTER: u32 = 0;

unsafe fn increment(){
    COUNTER += 1;
}

#[test]
fn test_unsafe(){
    unsafe {
        increment();
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    }
}
