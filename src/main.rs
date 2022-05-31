
use std::borrow::Cow;

fn modulo<'a>(input: &'a u16, base: &'a u16) -> Cow<'static, str> {
    let remainder = input % base;
    match remainder {
      0 => format!("No remainder").into(),
      _ => format!("Remainder: {}", remainder).into(),
    }
}


fn main() {
    let mut modulus = String::new();
    let mut n = String::new();

    println!("Enter a 16 bit integer value N");
    std::io::stdin().read_line(&mut n).unwrap();
    println!("Enter the 16 bit modulus value");
    std::io::stdin().read_line(&mut modulus).unwrap();

    println!("N = {}", &n);
    println!("mod = {}", &modulus);

    let N: u16 = n.trim().parse().unwrap();
    let divisor: u16 = modulus.trim().parse().unwrap();

    println!("N = {}", N);
    println!("mod = {}", divisor);
    let result = N % divisor;
    println!("Result = {}", modulo(&N, &divisor));
}
