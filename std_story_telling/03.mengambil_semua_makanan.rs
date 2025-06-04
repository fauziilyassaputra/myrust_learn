fn main() {
    println!("Hello, world!");
}

/* 
- cerita :
     siang telah tiba, waktunya makan siang . ayo kita keluarkan makanan yang berada di kotak . aku sudah tidak
     sabar nih!

- tugas :
   keluarkan isi kotak makanannya satu demi satu
 */


#[test]
fn test(){
    let mut kotak_makanan: [&str; 6] = ["_"; 6];
    kotak_makanan[0] = "pisang";
    kotak_makanan[1] = "apel";
    kotak_makanan[2] = "durian";


    for makanan in kotak_makanan{
        println!("{makanan}")
    }
    /*
    output :
        pisang
        apel
        durian
        _
        _
        _
    */
}

/*
- penjelasan :
 dengan menggunakan for, kita berhasil memecah array didalam kotak_makanan menjadi satu demi satu


*/
