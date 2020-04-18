fn main() {
    let mut v: Vec<i32> = Vec::new();
    for i in 3000..5001{
        if i%7==0||i%10==0{
            v.push(i);
        }
    }
    println!("{:?}",v )
}
