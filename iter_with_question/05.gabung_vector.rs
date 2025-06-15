fn main() {
    println!("Hello, world!");
}

/* 
 soal :
 1. gubungkan 2 variable vector menjadi satu vector!
 
 */
#[test]
fn test_iterator() {
    let vector1 = [1,2,3,4,5];
    let vector2 = [6,7,8,9,10];
    
    let gabung_vector: Vec<_> = vector1.iter().chain(vector2.iter()).collect();
    println!("{:?}", gabung_vector);
    // output : [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]


    let number = vec![1,2,3,4,5];
    let word = vec!["kucing", "tikus", "rubah", "srigala", "rakun"];
    let all_data: Vec<_> = number.iter()
        .zip(word.iter())
        .map(|(&num, &word)| format!("{num} adalah {word}"))
        .collect();
    println!("{:?}", all_data);
    //output : ["1 adalah kucing", "2 adalah tikus", "3 adalah rubah", "4 adalah srigala", "5 adalah rakun"]
}

/*
penjelasan :
1. zip digunakan untuk menggabungkan dua iterator  menjadi satu iterator. iterasi akan berhenti ketika salah 
satu iterator habis.
*/
