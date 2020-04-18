fn main() {
    let bike=vehicles::bikes(String::from("40km/l"));
    let  car=vehicles::cars(String::from("20km/l"));
    let truck=vehicles::trucks(String::from("10km/l"));
}
enum vehicles{
    bikes(String),
    cars(String),
    trucks(String)
}