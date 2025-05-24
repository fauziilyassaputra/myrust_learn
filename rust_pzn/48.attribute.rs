fn main() {
    println!("Hello, world!");
}

// derive attribute

#[derive(Debug, PartialEq, PartialOrd)]
struct Company{
    name: String,
    location: String,
    website:String
}

#[test]
fn test_attribute_derive(){
    let company = Company{
        name: "HSO section_6".to_string(),
        location: "new eridu".to_string(),
        website:"HSO.com/section/6".to_string()
    };
    println!("{:?}", company);
    let company2 = Company{
        name: "HSO section_6".to_string(),
        location: "new eridu".to_string(),
        website:"HSO.com/section/6".to_string()
    };

    let result =  company == company2;
    println!("{}", result);
    let result =  company > company2;
    println!("{}", result);
}

