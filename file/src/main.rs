use std::fs::File;

fn main() {
    let f = File::open("abc.txt");
    println!("{:?}", f);
    match f{
      Ok(data) => data,
      Err(error_kind) => panic!("error is: {:?}", error_kind),
 };
}
