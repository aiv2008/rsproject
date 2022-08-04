mod my_skip_list;
fn main() {
    let mut sl = my_skip_list::SkipList::new();
    let v = vec![9,8,7,6];
    for  i in &v{
        sl.insert(*i);
    }
    println!("hello world");
    println!("{:?}", sl);
}
