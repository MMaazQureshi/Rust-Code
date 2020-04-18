use std::io;
fn main() {
    let mut v: Vec<i32> = Vec::new();
    
    for i in 1..6{
        let mut a=String::new();
        io::stdin().read_line(&mut a).expect("incorrect input");
       let a=a.trim().parse().unwrap();

        v.push(a);
        

    };
    println!("{:?}",v)
}
