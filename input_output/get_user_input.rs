use std::io::stdin;
fn main() {
    let mut s = String::new();
    println!("enter your input");
    stdin().read_line(&mut s).unwrap();
    println!("input is {}", s)
}