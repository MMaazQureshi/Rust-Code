fn main() {
    let mut x=2;
    let mut f=|| x=x+2;
    f();
    println!("{}",x);

}
