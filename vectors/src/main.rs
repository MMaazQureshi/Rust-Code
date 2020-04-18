fn main() {
   let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(5) {
    Some(date) => println!("The third element is {}", date),
    None => println!("There is no {} element.",5),
}
}
