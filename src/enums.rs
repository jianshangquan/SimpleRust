pub enum ComputerLanguage{
    Java,
    Cpp,
    Csharp,
    Golang,
    Dart
}


pub fn language(language:ComputerLanguage) {  
 match language {
    ComputerLanguage::Dart=> println!("Dart language"),  
    ComputerLanguage::Cpp=> println!("C++ language"),  
    ComputerLanguage::Java=> println!("Java language"),  
    ComputerLanguage::Golang=> println!("Go language"),  
    ComputerLanguage::Csharp=> println!("C# language")
 }  
}  