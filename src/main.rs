fn main()  {
    
}

pub fn euclidean(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }   
    return a;
}

// Substraction-based version

// pub fn euclidean(mut a: i32, mut b: i32) -> i32 {
//     while a != b {
//         if a > b {
//             a -= b;
//         }
//         else {
//             b -= a;
//         }
//     }
//     return a;
// }