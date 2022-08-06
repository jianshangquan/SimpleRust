mod structs;
mod enums;
fn main() {
    println!("Hello, world!");
    let student = structs::init();
    student.go();

    let lang = enums::ComputerLanguage::Cpp;
    enums::language(lang);
}