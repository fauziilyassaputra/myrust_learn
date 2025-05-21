fn main() {
    println!("Hello, world!");
}

struct Person{
    first_name: String,  
    middle_name: String,
    last_name: String,
    age: u8
}

/*
- biasakan menulis nama struct diawali dengan huruf besar
- nama-nama pada setiap data disebut field 
*/

// membuat instance dari struct
/*
- setelah membuat definisi structnya, kita membuat instance /  value dari struct yang sudah kita buat (atau bisa disebut objek dalam bahasa oop)
- saat membuat instance dari struct, kita wajib menentukan semua value untuk field dari struct-nya
-berbeda dengan tuple, struct tidak harus diurutkan ketika memanggilnya
*/

#[test]
fn test_struct_person(){
    let person: Person = Person{
        age: 21,
        first_name: String::from("alex"),
        middle_name: String::from("andrina"),
        last_name: String::from("sebastane")
    };

    println!("first name : {}", person.first_name);
    println!("middle name :{}", person.middle_name);
    println!("last name :{}", person.last_name);
    println!("age :{}", person.age);
}


// Struct in function
/*
- struct sama seperti data lainnya, kita bisa gunakan dimanapun
- kita bisa gunakan struct sebagai parameter di function, atau return value di function
*/

fn print_person(person: &Person){
    println!("first name : {}", person.first_name);
    println!("middle name :{}", person.middle_name);
    println!("last name :{}", person.last_name);
    println!("age :{}", person.age);
}

#[test]
fn test_fucn_struct_person(){
    let person: Person = Person{
        age: 21,
        first_name: String::from("alex"),
        middle_name: String::from("andrina"),
        last_name: String::from("sebastane")
    };
    print_person(&person);
}

// Init Shorthand
/*
- kadang kita ingin membuat value untuk field pada struct dengan variable yang sudah ada, maka jika nama variable sama dengan nama field, kita tidak perlu sebutkan nama field secara eksplisit
- ingat, melakukan ini akan merubah ownershipnya
*/

struct Friends{
    first_name: String,
    last_name:String,
    age: u8
}

fn print_friend(friend : &Friends){
    println!("{}", friend.first_name);
    println!("{}", friend.last_name);
    println!("{}", friend.age);

}

#[test]
fn init_shorhand(){
    let first_name = String::from("anby");
    let last_name = String::from("Demara");

    let friend: Friends = Friends{
        age: 21,
        first_name,
        last_name
    };
    print_friend(&friend);
}


// Sturct update syntax
/*
beberapa problem :
- saat menggunakan struct update syintax, maka ownershipnya secara otomatis dipindahkan ke field instance yang baru
- maka instance yang lama tidak bisa digunakan karena valu di fieldnya sudah dipindahkan 
-jika memang tidak diinginkan terjadi perpindahan ownership, maka lakukan clone pada data fieldnya

*/


#[test]
fn update_syntax(){
    let person: Person = Person{
        age: 21,
        first_name: String::from("alex"),
        middle_name: String::from("andrina"),
        last_name: String::from("sebastane")
    };
    print_person(&person);

    let person2: Person = Person{.. person};
    print_person(&person2);

}

// clone 
#[test]
fn update_syntax_clone(){
    let person: Person = Person{
        age: 21,
        first_name: String::from("alex"),
        middle_name: String::from("andrina"),
        last_name: String::from("sebastane")
    };
    print_person(&person);

    let person2: Person = Person{
        first_name: person.first_name.clone(),
        middle_name: person.middle_name.clone(),
        last_name: person.last_name.clone(),
        ..person // age adalah fix size, sehingga tak pelu di clone karena datanya di copy bukan dipindahkan
    };
    print_person(&person2);

}

// tuple struct
struct GeoPoint(f64,f64);

#[test]
fn tuple_struct(){
    let geo_point = GeoPoint(-6.232223,  100.2323);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);

}

// nothing struct
struct Nothing;
#[test]
fn struct_nothing(){
    // gunakan _ pada awal variable agar tidak terdeteksi warning
    let _nothing1 = Nothing;
    let _nothing2 = Nothing;

}
