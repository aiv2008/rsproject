fn main() {
    let a: i32 = 5;
    let v = vec![34,5,6,7,8,99,3];
    println!("{}", v[(a-1) as usize]);
    for  i in &v[..((a-1) as usize)]{
        println!("{}", *i);
    }
}
