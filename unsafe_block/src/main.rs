#[derive(Debug)]
struct Node(i32);

fn push(elem: i32){
    let mut node  = Node(elem);
    let mut p_str = Option::Some(&node as *const Node);
    println!("{:?},{:?}",node, p_str);
}

use Option;

fn index(idx: usize, arr: &[u8] )->Option<u8>{
   if idx <= arr.len(){
      unsafe{
        Some(*arr.get_unchecked(idx))
      } 
   } else{
        None
   }
   //unimplemented!();
}

fn main() {
    //push(2);
    //push(3);
    //let s = String::from("hello world");
    //let usf_s: *const String = &s as *const String;
    //println!("{:?}",  usf_s);
    //let mut usf_s_2 = (usf_s as usize) + 4;
    //println!("{:?},{:?}",usf_s, (usf_s_2 as *const String));
}

