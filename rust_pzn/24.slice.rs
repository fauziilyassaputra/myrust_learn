fn main() {
    println!("Hello, world!");
}

#[test]
fn slice_reference(){
    let array: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    
    let slice1: &[i32] = &array[..]; //Rangefull
    println!("slice1 = {:?}", slice1);

    let slice2: &[i32] = &array[0..5]; //Range
    println!("Slice2 = {:?}", slice2);

    let slice3: &[i32] =  &array[5..]; //Range from
    println!("slice3 : {:?}", slice3);
}
