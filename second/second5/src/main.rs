fn main() {
   let Circle=shape::circle(3);
   let data=Circle.returnshape();
    println!("{:?}",data);
     let triangle=shape::triangle(3,5,3);
   let data=triangle.returnshape();
    println!("{:?}",data);
     let Rectangle=shape::circle(3);
   let data=Rectangle.returnshape();
    println!("{:?}",data);
}
#[derive(Debug)]
enum shape{
    circle(u32),
    triangle(u32,u32,u32),
    rectangle(u32,u32,u32,u32),
    square(u32,u32,u32,u32),
}
impl shape{
    fn returnshape(&self)->&shape{
        self
    }
}