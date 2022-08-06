pub struct Student{
    name: String,
    age: u8,
    active: bool
}


impl Student {
    pub fn go(&self){
        println!("attend school")
    }
    
    pub fn leave(&self){
        println!("leave school");
    }
}


pub fn init() -> Student{
    let student: Student = Student{
        name: String::from("jainshangquan"),
        age: 20,
        active: true
    };

    println!("{}", student.name);
    return student;
}