fn main()  {
    let x = euclidean(210, 45);
    println!("{x}");
}

fn euclidean(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }   
    return a;
}