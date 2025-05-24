fn main() {
    println!("Hello, world!");
}

#[test]
fn test_daggling_reference(){
    let r: &i32;
    {
        let _x = 5;
        // r = &x; // error 
    }
    r = &40;
    println!("{}", r);
}

/*
fn longest(value1: &str, value2: &str) -> &str {
    if value1.len() > value2.len() {
        value1
    }else {
        value2
    }
     /*
    akan  terjadi error karena membingungkan, value1 atau value2 sebenarnya yang mau dipanggil ? , rust melakukan demikian agar mencegah daggling reference agar memorinya safety
     */
}

*/

   

// Lifetime Anotation
/*
- rust menyediakan fitur bernama life annotation , dimana kita bisa menyebutkan yang mana yang kemungkinan akan di borrow
- cara menambahkan Lifetime Annotation sama seperti generic, hanya saja typenya di awali dengan petik satu
 
*/

fn longest<'a>(value1: &'a str, value2: &'a str) -> &'a str {
    if value1.len() > value2.len() {
        value1
    }else {
        value2
    }
}

#[test]
fn test_lifetime_anotation(){
    let value1 = "hoshimi";
    let value2 = "miyabi";
    let result = longest(value1, value2);
    println!("{}", result);
}

/*
- lifetime annotation tidak mengubah waktu hidup, hanya penanda untuk membantu rust borrow checker
- di lifetime annotation bisa terjadi dangling reference
*/

#[test]
fn test_lifetime_annotation_dangling_reference(){
    let value1 = String::from("jane");
    let value2 = String::from("doe"); // not error 
    let result;
    {
        // let value2 = String::from("doe"); //error 
        result = longest(value1.as_str(), value2.as_str());
    }
    println!("{}", result);
}

// life annotation pada struct

struct Student<'a, 'b>{
    name: &'a str,
    last_name: &'b str
}

// lifetime annotation di method
impl<'a,'b> Student<'a,'b> {
    fn longest_name(&self, student: &Student<'a, 'b>) -> &'a str {
        if self.name.len() > student.name.len(){
            self.name
        } else {
            student.name
        }
    }
}



fn longest_student_name<'a, 'b>(student1: &Student<'a,'b>, student2: &Student<'a, 'b>) -> &'a str {
    if student1.name.len() > student2.name.len() {
        student1.name
    } else {
        student2.name
    }
}

#[test]
fn test_student(){
    let student = Student{
        name: "ellen",
        last_name: "joe",
    };
    println!("{}", student.name);

    let student2 = Student {
        name: "koleda",
        last_name: "belobog"
    };
    let result = longest_student_name(&student, &student2);
    println!("{}", result);

    let result = student.longest_name(&student2);
    println!("{}", result);
}

// lifetime digabungkan dengan generic

struct Teacher<'a, ID> where ID: Ord {
    id: ID,
    name: &'a str
}

#[test]
fn test_lifetime_annotation_generic(){
    let teacher = Teacher{
        id: 10,
        name: "harumasa"
    };
    println!("{}",teacher.id);
    println!("{}", teacher.name);
}
