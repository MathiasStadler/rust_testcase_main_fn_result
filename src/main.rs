use std::error::Error;

fn read_i32() -> Result<i32, Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let x = input.trim_end().parse::<i32>()?;
    Ok(x)
}

fn main() {
    println!("{:?}", read_i32());
}