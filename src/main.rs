// use std::error::Error;

// fn read_i32() -> Result<i32, Box<dyn Error>> {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input)?;
//     let x = input.trim_end().parse::<i32>()?;
//     Ok(x)
// }

// fn main() {
//     println!("{:?}", read_i32());
// }

fn get_result() -> Result<String, String> {
    Ok(String::from("foo")) // <- works fine
    //Result::Err(String::from("Error"))
 }
 
 fn main(){
     match get_result(){
         Ok(s) => println!("{}",s),
         Err(s) => println!("{}",s)
     };
 }
 