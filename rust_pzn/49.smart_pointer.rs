fn main() {
    println!("Hello, world!");
}

#[test]
fn test_box(){
    let value: Box<i32> = Box::new(10);
    println!("{}", value);
    display_number(*value);
    display_number_reference(*value);
}

fn display_number(value: i32){
    println!("{}", value)
}
fn display_number_reference(value: i32){
    println!("{}", value)
}

// recursive data type

/*
box akan sangat berguna ketika kita menemui tipe data yang recursive
*/

#[derive(Debug)]
enum ProductCategory {
    Of(String, Box<ProductCategory>),
    End
}

#[test]
    fn test_box_enum() {
        let category = ProductCategory::Of(
            "laptop".to_string(),
            Box::new(ProductCategory::Of(
                "Dell".to_string(),
                Box::new(ProductCategory::End)
            )),
        );
        println!("{:?}", category);
        print_category(&category);
    }

fn print_category(category: &ProductCategory) {
    println!("{:?}", category);
}
