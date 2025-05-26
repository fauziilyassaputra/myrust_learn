fn main() {
    println!("Hello, world!");
}


/*
- Rust menggunakan ownersip untuk melakukan data management memory
- ownership adalah fitur mengolah data dalam memori tanpa harus adanya fitur garbage collection 
atau manual memory management
*/

#[test]
fn ownership_rules(){
    // a tidak bisa diakses disini, belum di deklarasikan
    let a = 10; // a bisa diakses mulai disini

    { // b tidak bisa diakses disini,  belum dideklerasikan
        let b = 20; // b bisa diakses mulai disini
        println!("{}", b);
    } // scope  selesai , b dihapus, b tidak bisa diakses lagi

    println!("{}", a);
    // scope a selesai , a dihapus, a tidak bisa diakses lagi
}


// Data copy
/*
- setiap value harus dimiliki oleh satu owner pada satu waktu
- setiap kita berinteraksi dengan data, maka data akan dimiliki hanya satu owner
- semua data yang bersifat fixed size (yang disimpan di stack), keika kita tambahkan ke variable baru (owner baru), maka hasilnya adalah data akan di copy, sehingga variable baru (owner baru) akan memiliki data
hasil copy dari variable lama (owner lama)
*/

#[test]
fn data_copy(){
    let a = 10;
    let b = a; // copy data dari a ke b 

    println!("{} {}", a, b)
/*
- function data_copy terletak didalam stack beserta isinya
- dalam function data_copy, variable b mengcopy isi variable a yaitu 10 . sehingga jika variable b
di ubah tidak memiliki dampak apapun pada variable a
*/

}

// Ownership movement
/*
- Data copy tidak terjadi untuk tipe data yang disimpan di heap
- dalam satu value hanya memiliki satu owner
- ketika kita coba buat variable baru(owner baru) dari variable lama(owner lama), maka yang terjadi 
bukanlah copy, melainkan transfer ownership dari owner lama ke owner baru
- setelah proses trandfer selesai, secara otomatis owner lama akan dianggap tidak valid lagi digunakan
*/

#[test]
fn ownership_movement(){
    let name1 = String::from("hoshimi");

    // ownership dari name1 dipindahkan ke name2
    let name2 = name1;
    // name1 tidak bisa diakses disini

    println!("{}", name2);
    // println!("{}", name1); //error

}

// Clone (cara untuk 'mengcopy' data heap)

#[test]
fn my_clone(){
    let name1 = String::from("shoukaku");
    let name2 = name1.clone();

    println!("{} {}", name1, name2);
}
    
