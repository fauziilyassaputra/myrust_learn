fn main() {
    println!("Hello, world!");
}

fn full_name(first_name: &String, last_name: &String)->  String{
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name(){
    let first_name = String::from("koleda");
    let last_name = String::from("Belobog");

    let full_name = full_name(&first_name, &last_name);

    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

// Borrowing 

// fn change_value(value: &String){
//     value.push_str("test")
// }

// #[test]
// fn test_change_value(){
//     let mut value = String::from("hoshimi");
//     // change_value(&value); //error (valuenya adalah reference, sehingga  klw di borowing(dipinjam) tak bisa di ubah2 seperti menjadi mutable meskipun asalnya mutable)
//     println!("{}", value);
// }


// mutable reference
/*

-  menggunakan mutable reference, variable owner juga harus mutable
-  untuk menjamin keamaan, dalam satu waktu hanya diperbolehkan membuat satu mutable variable reference

*/

fn mutable_value(value: &mut String){
    value.push_str(" Test");
}

#[test]
fn test_reference_mutable(){
    let mut value = String::from("hoshimi");
    mutable_value(&mut value);
    mutable_value(&mut value);
    mutable_value(&mut value);
    println!("{}", value);
}

/*
- pada kode di atas, walaupun terlihat lebih dari satu penggunaan, sebenarnya kode diatas tetap di jalankan
dalam satu waktu dan berada di scope 'mutable_variable'  yang terpisah dari scope variable yang ada. 
- saat data berada pada function mutable_variable , setelah parameter dipanggil akan otomatis terhapus 
karena sudah keluar dari scope functionnya sehingga data variable berikutnya bisa dijalankan.
*/


#[test]
fn test_reference_mutable2(){
    let mut value = String::from("hoshimi");
    
    let valueBorrow1  =  &mut value;
    // let valueBorrow2 = &mut value; //  Error

    /*
    variable 'valueBorrow2' error karena berada pada 1 lifescyle yang sama 
     */

    mutable_value(valueBorrow1);
    // mutable_value(valueBorrow2);

    println!("{}", value);
}


// Dangling pointer



fn get_full_name(first_name: &String, last_name: &String)->  &String{
    let name =  format!("{} {}", first_name, last_name);
    return &name;
    //  name itu hanya berada pada function ini, sehingga 'name' akan otomnatis dihapus ketika 
    // function selesai dipanggil , jadinya percuma memanggil returnnya
}

#[test]
fn get_test_full_name(){
    let first_name = String::from("koleda");
    let last_name = String::from("Belobog");

    let full_name = full_name(&first_name, &last_name);

    println!("{}", full_name);
    
}
/*
- solusi untuk masalah dangling pointer , jika memang data yang dikembalikan dibuat dalam bentuk function
maka kita harus kembalikan dalam bentuk value langsung, bukan reference
*/



fn get_full_name2(first_name: &String, last_name: &String)->  String{
    let name =  format!("{} {}", first_name, last_name);
    return name; // (let full_name yang menjadi return name pada function ini)
    
}

#[test]
fn get_test_full_name2(){
    let first_name = String::from("koleda");
    let last_name = String::from("Belobog");

    let full_name = full_name(&first_name, &last_name);

    println!("{}", full_name);
    
}
