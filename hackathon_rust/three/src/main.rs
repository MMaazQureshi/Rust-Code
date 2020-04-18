fn main() {
   let arr=[1,2,3,4,5];
   let num=3;
   for i in arr.iter(){
       if i==&num{
           println!("num {} is present",num );
       }
   }
}
