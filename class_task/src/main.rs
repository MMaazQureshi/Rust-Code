fn main() {
    println!("Hello, world!");
}
fn print_string(stri:&String,num:i32){
    for i in 0..num{
        println!("{}",stri);
    }
}
fn find_vowels(stri:&String){
    let mut vowel_count=0;
    for i in stri.iter(){
        if i=='a'||i=='e'||i=='i'||i=='o'||i=='u'{
            vowel_count=vowel_count+1;
        }

        }
    }
    fn find_vowels_consonents(stri:&String){
    let mut vowel_count=0;
    let mut consonent_count=0;
    for i in stri.iter(){
        if i=='a'||i=='e'||i=='i'||i=='o'||i=='u'{
            vowel_count=vowel_count+1;
        }
        else{

        }

        }
    }


