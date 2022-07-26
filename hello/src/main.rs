fn main() {
    //let a: i32 = 5;
    //let v = vec![34,5,6,7,8,99,3];
    //println!("{}", v[(a-1) as usize]);
    //for  i in &v[..((a-1) as usize)]{
    //    println!("{}", *i);
    //}
    let mut a = 3;
    a = f(a);
    println!("{}", a);
}

fn f(mut a: usize)->usize{
    a+=1;
    a
}
