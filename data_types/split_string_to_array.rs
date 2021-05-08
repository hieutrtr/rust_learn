fn main() {
    let a = "18,19,20,18,19";
    let _a: Vec<i32> = a.split(",").into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    print!("a: {:?}", _a);
}