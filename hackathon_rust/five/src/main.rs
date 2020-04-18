fn main() {
    
}
struct Quarter1{
    name:String,
    roll_number:i32,
    test1marks:i32,
    test2marks:i32,
    test3marks:i32,
}
impl Quarter1
{
    fn avg(&self)->i32{
        (self.test1marks+self.test2marks+self.test3marks)/3
    }
    fn max(&self){
        if self.test1marks>self.test2marks&&self.test1marks>self.test3marks
        {
            println!("test marks 1 is max")
        }
        else if self.test2marks>self.test1marks&&self.test2marks>self.test3marks
        {
            println!("test marks 3 is max")
        }
        else{
            println!("test marks 4 is max")
        }
    }

}
