fn main() {
    println!("Hello, world!");
}

/* 
 soal :
 pada variable data tedapat banyak angka nol, ambil nolnya!
 
 */

use std::collections::VecDeque;

#[test]
fn hilangkan_whitespace() {
    
   let mut data: VecDeque<i32> = vec![0,1,2,3,4,5,0,0].into();
    data.pop_front();
    data.pop_back();
    data.pop_back();
    
    println!("{:?}", data);
}
/*


 */
