fn main() {
    println!("Hello, world!");
}

/* 
 soal :
  buatkan vec seperti [1,0,0,1] dengan push_back hanya 1 kali !
 
 */


#[test]
fn mengolah_data() {
    let mut data = VecDeque::new();
    data.push_front(0); // [0]
    data.push_front(0); // [0,0]
    data.push_front(1); // [1,0,0]
  
    data.push_back(1); // [1,0,0,1]

  
    println!("{:?}", data);
    //output : [1,0,0,1]
    
   
}

/*


 */
