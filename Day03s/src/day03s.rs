fn main() {
    let a = vec![1,2,3];
    let b = vec![4,5,6];
    let d = f(&a,&b);
    let e = f(&a,&b);
    println!("{} {}", d, e);
}

fn f(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    a.iter().sum()
    +
    b.iter().sum()
}