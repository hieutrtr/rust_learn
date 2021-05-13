fn main() {
    // define a range
    let range = 1..10;

    // loop thru a range ( iter )
    for i in range {
        print!("{} ", i);
    }

    // define vector
    println!("\nfor iter vector");
    let v = vec![1,2,3,7,8,9];
    for i in v.iter() {
        print!("{} ", i);
    }

    println!("\nloop and match");
    let mut range1 = 1..9;
    // let v1 = vec![1,2,3,7,8,9]; // cannot be used with vector
    loop {
        match range1.next() {
            Some(x) => {
                print!("{} ", x)
            },
            None => {break}
        }
    }

    println!("\nvector ref loop");
    let nums = vec![1,2,3,6,5,4];
    for num in &nums {
        print!("{} ", num);
    }

    println!("\nskipped and take loop with vector");
    let nums1 = vec![1,2,3,4,5,6];
    for num in nums1.iter().skip(2).take(3) {
        print!("{} ", num);
    }
}