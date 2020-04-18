fn main() {
    let l=laptops::Lenovo;
    let d1=laptops::Dell(dell::series3000);
    let a= laptops::Asus;
    let d2=laptops::Dell(dell::series5000);
    
}
enum laptops{
    Dell(dell),
    Hp,
    Asus,
    Lenovo
}
function returnlaptop(&laptops){
    
}

enum dell{
    series1000 , 
    series2000 , 
    series3000 , 
    series4000 , 
    series5000 ,
    series6000 ,
}