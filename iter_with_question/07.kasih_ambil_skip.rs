fn main() {
    println!("Hello, world!");
}

/* 
 
 
 */
#[test]
fn test_iterator() {
  // memberi nomor pada masing2 sub vector
  let var_enum = ['a','b','c'];
  let check_enum: Vec<_> = var_enum.iter().enumerate().collect();
  println!("{:?}", check_enum);
  // output : [(0, 'a'), (1, 'b'), (2, 'c')]

  // melihat data tanpa mengosumsinya
  let data_rahasia = vec![1,2,3,4,5,6,7,8];
  let mut intip_data_rahasia = data_rahasia.iter().peekable();
  let intip1 = intip_data_rahasia.peek();
    
  println!("{:?}", intip1);
  // output : Some(1)

  // skip beberapa data
  let data_skip = [1,2,3,4,5,6,7,8,9];
  let check_data_skip: Vec<_> = data_skip.iter().skip(3).collect();
  println!("{:?}", check_data_skip);
  //output : [4, 5, 6, 7, 8, 9]

  //mengambil data
  let data_take = [1,2,3,4,5,6,7];
  let check_data_take: Vec<_> = data_take.iter().take(5).collect();
  println!("{:?}", check_data_take);
  //output : [1, 2, 3, 4, 5]

  
  
}
