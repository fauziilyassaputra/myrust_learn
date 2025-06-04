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
    let mut kotak_makanan: [i32; 6] = [0; 6];
    kotak_makanan[0] = 1;
    kotak_makanan[1] = 2;
    kotak_makanan[2] = 3;

    for makanan in kotak_makanan{
        println!("{makanan}")
    }
    /*
    output :
    1
    2
    3
    0
    0
    0
    */
}

/*
- penjelasan :
 dengan menggunakan for, kita berhasil memecah array didalam kotak_makanan menjadi satu demi satu


*/
