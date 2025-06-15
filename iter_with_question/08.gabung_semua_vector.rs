fn main() {
    println!("Hello, world!");
}

/* 
 soal:
 gabungkan seluruh vector yang ada pada variable data tumuk menjadi 1 variable!
 
 */

#[test]
fn test_iterator() {
    let data_tumpuk = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    let data_bersatu:Vec<_> = data_tumpuk.into_iter().flat_map(|x| x.into_iter()).collect();
    println!("{:?}", data_bersatu);
    //output : [1, 2, 3, 4, 5, 6, 7, 8, 9]
}
