use std::fmt::Display;



struct People{
    age: u128
}

impl Display for People {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "age {}", self.age)
    }
}

fn main() {
    println!("Hello, world!");
    // let student = structs::init();
    // student.go();

    // let lang = enums::ComputerLanguage::Cpp;
    // enums::language(lang);

    let mut outsideValue: String = String::from("123456");
    passValueFromOutside(&mut outsideValue);
    println!("{}", outsideValue);
}


fn passValueFromOutside(value: &mut String){
    *value = String::from("new string");
    println!("{}",value);
}