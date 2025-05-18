fn main() {
    println!("Hello, world!");
}

#[test]
fn array() {
    let my_array:[i32; 5] = [1,2,3,4,5];
    println!("array: {:?}", my_array);

    // index array start from 0
    let a = my_array[0];
    let b = my_array[1];
    println!("index 0 & 1 from my array: {} {}", a, b);
  

    // mutable array
    let mut my_mutable_array = [10,30,30,40];
    my_mutable_array[1] = 20;
    println!("index 1 from my mutable array has been changed: {:?}", my_mutable_array);


    // length array (not last index)
    let my_length_array = my_mutable_array.len();
    println!("length from my_mutable_array: {}", my_length_array)

    
}

// two demonsional array
#[test]
fn two_demonsional_array(){
    let matrix: [[i32; 3]; 2] = [
        [1,2,3],
        [4,5,6]
    ];

    println!("semua nilai : {:?}", matrix);
    println!("row 1 : {:?} ", matrix[0]);
    println!("row 2 : {:?} ", matrix[1]);

    println!("column from row 1 : {:?}",matrix[0][0]);
    println!("column from row 1 : {:?}",matrix[0][1]);
    println!("column from row 1 : {:?}",matrix[0][2]);

    println!("column from row 2 : {:?}",matrix[1][0]);
    println!("column from row 2 : {:?}",matrix[1][1]);
    println!("column from row 2 : {:?}",matrix[1][2]);

}
