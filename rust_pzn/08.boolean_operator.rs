fn main() {
    println!("Hello, world!");
}


/* 
&& = and (true && true = true, true && false = false, false && true = false , false && false = false
|| = or (true || true = true, true || false = true, false || true = true, false || false = false
! =  Not (!true = false, !false = true)
*/

#[test]
fn boolean_operator(){
    let absen = 70;
    let nilai_akhir = 80;

    let lulus_absen: bool = absen >= 75;
    let lulus_nilai_akhir:bool = nilai_akhir >= 75;

    let lulus = lulus_absen && lulus_nilai_akhir;
    println!("apakah lulus : {}", lulus);
}
