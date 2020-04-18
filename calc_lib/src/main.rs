//use extern crate::calculator;
mod calculator{
   pub mod Basic_functions{
      pub  fn Add(n1:i32,n2:i32)->i32{
            n1+n2
        }
        fn Subtract(n1:i32,n2:i32)->i32{
            n1-n2
        }
        fn Multiply(n1:i32,n2:i32)->i32{
            n1*n2
        }
        fn Divide(n1:i32,n2:i32)->i32{
            n1/n2
        }
}
  pub  mod power_functions{
        fn square_function(n:i32)->i32{
            n*n
        }
        fn cube_function(n:i32)->i32{
            n*n*n
        }
     pub   fn power_function(number:i32,power:i32)->i32{
            if power==0{
                return 1
            }
            else if power==1{
                return number
            }
            else{
                return number*power_function(power-1,number)
            }
        }

    }
}
fn main() {
   println!("{}", crate::calculator::Basic_functions::Add(5,7));
   println!("{}", crate::calculator::power_functions::power_function(5,3));
}

