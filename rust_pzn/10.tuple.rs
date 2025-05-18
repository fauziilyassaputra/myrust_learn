fn main() {
    println!("Hello, world!");
}

#[test]
fn tuple (){
    // default tuple is imutable
    let data: (i32,f64, bool) = (10, 10.5, true);
    println!("contoh tuple: {:?}", data);

    // the index of tuple start from 0
    let a = data.0;
    let b = data.1;  
    let c = data.2;
    println!("{} {} {}", a, b, c);

    // destructring tuple
    let (a,_,c)  = data;
    println!("with destructring : {}  {}", a,c);

    // mutable tuple
    let mut data_mutable_tuple = (10,25.5, true);

    data_mutable_tuple.0 =  20;
    data_mutable_tuple.1 = 20.5;
    data_mutable_tuple.2 = false;

    println!("data tuple yang mutable: {:?}", data_mutable_tuple);


}

 // unit (tuple with not anything value or empty)

 fn my_unit(){
    println!("hello world again")
 }

 #[test]
 fn test_unit(){
    let result: () = my_unit();
    println!("{:?}", result)
 }
