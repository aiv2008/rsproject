//fn push_vec<T>(v: Vec<T> )->Vec<T>{
//    let mut res_v: Vec<T> = Vec::new();
//    for i in &v {
//        res_v.push(*i);
//    }
//    res_v
//}

fn main() {
    //let mut v = vec![1,2,3,4,5];
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);
    v.push(7);
    let first = &v[0];
    //v.push(6);
    //let first = &v[0];
    println!("The first element is: {}", first);
    //v.push(6);
}


//pub struct Vec<T> {
//    ptr: *mut T,
//    cap: usize,
//    len: usize,
//}
//
//impl <T> Vec<T>{
//
//}
