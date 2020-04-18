fn main() {
    let stud= student::std(String::from("maaz"),5252,gender::male);
    let passout= student::passout(String::from("abc"),String::from("2012"),gender::male);
    
}
enum student{
    std(String,u32,gender),
    passout(String,String,gender)
}
enum gender{
    male,
    female,
}
