use std::io;
fn main() {
    let mut num = String::new();
io::stdin().read_line(&mut num)
.expect("Failed to read line");
let num: u32 = num.trim().parse()
.expect("Please type a number!");
println!("enter 1 to multiply and 2 to sum");
let mut op = String::new();
io::stdin().read_line(&mut op)
.expect("Failed to read line");
let op: u32 = op.trim().parse().unwrap();
for i in 1..num+1
{
        print!("{}", i)
    }
    println!("", );
if op==1{
    let mut sum=0;
    
     for i in 1..num+1{
        sum+=i;
    }
    println!("sum= {}",sum )
}
else if op==2{
    let mut prod=1;
    
     for i in 1..num+1{
        prod*=i;
    }
    println!("product= {}",prod )

}
}
