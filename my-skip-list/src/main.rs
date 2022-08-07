mod my_skip_list;
use rand::{prelude::*, Rng};
fn main() {
    //let rand_pos =  (rand::thread_rng()).gen_range(1..3);
    //println!("{}", rand_pos);
    let mut sl = my_skip_list::SkipList::new();
    let v = vec![9,8,7,6,45,21,1,23];
    for  i in &v{
        sl.insert(*i);
        println!("sl.head.len={}", sl.len());
    }
    println!("hello world");

    sl.iter();
}
