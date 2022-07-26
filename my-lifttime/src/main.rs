//use std::{slice::from_raw_parts, str::from_utf8_unchecked};
//use std::fmt::Debug;
//
//fn get_memory_location()->(usize, usize){
//    let s = "hello world";
//    let ptr = s.as_ptr() as usize;
//    let len = s.len();
//    (ptr, len)
//}
//
//fn get_str_at_loction(pointer: usize, length: usize)-> &'static str{
//    //"hello word"
//    unsafe {
//        println!("{:?}", pointer as *const u8);
//        from_utf8_unchecked(from_raw_parts(pointer as *const u8, length))
//    }
//}
//
//fn print_it<T: Debug + 'static>( input: &T) {
//    println!( "'static value passed in is: {:?}", input );
//}
//
//fn print_it1( input: impl Debug + 'static ) {
//    println!( "'static value passed in is: {:?}", input );
//}

fn main() {
    //let (ptr, len) = get_memory_location();
    //let s = get_str_at_loction(ptr, len);
    //println!("s={}", s);
    //let i = 5;
    //print_it(&i);

    //let mut data = vec![1, 2, 3];
    //let x = &data[0];
    ////println!("{}", x);
    //data.push(4);
    //println!("{}", x);

    //一个特别有趣的语法糖是，每个let语句都隐含地引入了一个作用域。在大多数情况下，这其实并不重要。
    //然而，这对那些相互引用的变量来说确实很重要。作为一个简单的例子，让我们对这段简单的 Rust 代码进行完全解糖：
    //'a: {
    //    let x: i32 = 0;
    //    'b: {
    //        let y: &'b i32 = &'b x;
    //        'c :{
    //            let z: &'c (&'b i32) = &'c y;
    //        }
    //    }
    //}
    //
    'a: {
        let x: i32 = 0;
        'b:{
            let z : &'b i32';
            'c :{
                let y: &'c i32' = &'c x;
                z = y;
            }
        }
    }
}
