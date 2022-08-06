pub struct Student{
    name: String,
    age: u8,
    active: bool
}


pub fn init(){
    let student: Student = Student{
        name: String::from("jainshangquan"),
        age: 20,
        active: true
    };

    println!("{}", student.name);
}