// make a struct Post
// new notification trait method returns hey you have got new notification
// -summarize  trait method returns this is the summary of this Post
// -notify() function should accept a string onli if struct has both of these traits implemented  

fn main() {
    
}
struct post {
    author:String,
    topic :String
}
pub trait summary{
    fn summarize(&self)-> String;
}

pub trait newnotification{
    fn notify(&self)-> String;
}
impl newnotification for post{
    fn notify(&self)->String{
        format!("New notification is here")
    }
}
impl summary for post{
    fn summarize(&self)->String{
        format!("New notification is here")
    }
}
pub struct Post {
pub Author: String,
pub Topic: String,
}
impl Summary for Post{
    fn summarize(&self) -> String {
format!("{}, by {} )", self.Author, self.Topic)
}
fn newnotification(&self)->String{
    String::from("Hey you have got a notification")

}
}
fn notif(){

}
