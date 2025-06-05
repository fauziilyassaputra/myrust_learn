fn main() {
    println!("Hello, world!");
}

/* 
   - cerita:
       pagi hari tiba, sebelum berangkat. kami mengumpulkan bekal di sekitar kami. ery memancing sedangkan aku mencari buah-buahan 
     yang ada. 
     setelah 1 jam. kami berhasil mendapatkan 5 apel, 3 pisang, dan 2 ikan. ery tak sengaja mendapatkan ransel kecil ketika
     memancing . diapun mengeringkannyadan benercana mengeringkannya. sementara semua makanan diletakkan diransel utama sampai
     ransel yang baru ditemukan kering dan siap dipakai.

    - masalah:
    1.berapa jumlah makanan didalam ransel utama setelah semua  makanan dimasukkan ke dalamnya ?
    2.masukkan beberapa makanan dari ransel utama ke ransel kecil setelah kering
 */

#[test]
fn test2(){
    // isi ransel
    let mut isi_ransel = vec!["apel","apel","apel","apel","apel","pisang","pisang","pisang","ikan","ikan","ikan"];
    println!("isi ransel: {:?}", isi_ransel);
    // isi ransel: ["apel", "apel", "apel", "apel", "apel", "pisang", "pisang", "pisang", "ikan", "ikan", "ikan"]

    // cek total isi ransel
    let total_isi_ransel = isi_ransel.len();
    println!("total isi ransel: {}", total_isi_ransel - 1);
    //  total isi ransel: 10

    // berbagi dengan ransel kecil
    let isi_ransel_kecil = isi_ransel.split_off(8);
    println!(" isi ransel utama: {:?}", isi_ransel);
    println!(" isi ransel kecil: {:?}", isi_ransel_kecil);
    //  isi ransel utama: ["apel", "apel", "apel", "apel", "apel", "pisang", "pisang", "pisang"]
    //  isi ransel kecil: ["ikan", "ikan", "ikan"]

}

/* 
 -penjelasan :
   1.len() menghitung total panjang index vector hingga dihitung dari nol. untuk mengatasinya, kita kurangi satu dari total indexnya.
   2.split_off(index) memisahkan vec menjadi dua bagian, split_off(index) akan mengambil dari index terakhirmya
   3.perbedaannya dengan drain(), drain() menghapus elemen dalam range tertentu sedangkan split_off() itu memisahkan menjadi
   dua bagian berdasarkan index yang diberikan
*/
