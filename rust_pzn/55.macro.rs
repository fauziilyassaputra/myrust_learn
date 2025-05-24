fn main() {
    println!("Hello, world!");
}

// macro (!)

macro_rules! hi {
    () => {
        println!("hi")
    };
    ($name: expr) =>{
        println!("Hi {}", $name)
    }
}

#[test]
fn test_macro(){
    hi!();
    hi!("hoshimi");
    hi!(
        "hoshimi"
    )
}

// Repitition 
/*
- jika kita butuh parameter macro lebih dari satu, maka kita pakai repitition
-ketika menggunakan macro, kita bisa gunakan $() diikuti dengan koma, lalu repetion operator :
* = boleh berapapun
+ = boleh berapapun dan minimalnya satu
? = boleh satu atau kosong, sehingga tidak perlu pemisah koma

*/

macro_rules! iterate {
    ($array: expr) => {
        for i in $array {
            println!("{}", i);
        }
    };
    ($($item: expr),*)  => {
        $(
            println!("{}", $item);
        )*
    }
}

#[test]
fn test_macro_iterate(){
    iterate!([1,2,3,4,5,6,7,8,9,10]);
    iterate!(1,2,3,4,5,6,7,8,9,10);

}
