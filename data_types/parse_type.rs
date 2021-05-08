fn main() {
    let n = "5";
    let a = "18,19,20,18,19";
    let _n: i32 = n.parse().unwrap();
    let _a: Vec<i32> = a.split(",").into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    print!("n: {}", _n);
    print!("a: {:?}", _a);
}