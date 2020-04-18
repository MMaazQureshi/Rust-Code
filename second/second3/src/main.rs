fn main() {
    let stud= student{
        name:String::from("maaz"),
        RollNo:334,
        gender:gender::male,
    };
}
enum gender{
    male,
    female,
}
struct student{
    name:String,
    RollNo:u32,
    gender:gender,
}
