fn main() {
    println!("Hello, world!");
}

/* 
 
 
 */

#[test]
fn test_iterator() {
    let vector = vec![1,2,3,4,5,6,7,8,9,10,11,12];
    let iter = vector.iter().nth(3);
    println!("{:?}", iter);
    // output : Some(4)

}
