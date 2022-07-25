//结构：
//[ptr]->[elem, ptr]->[null]

use std::ptr;
use std::{
    alloc::{dealloc, Layout},
    ptr::drop_in_place,
};

#[derive(Debug)]
struct Stack<T> {
    //head: Link<T>,
    head: *mut Node<T>,
}

//type Link<T> = *mut Node<T>;

#[derive(Debug)]
struct Node<T>{
    elem: T,
    //next: Link<T>,
    next: *mut Node<T>,
}

impl <T>  Drop for Node<T>{
    fn drop(&mut self){
        println!("drop current node!");
    }
}

impl <T: std::fmt::Debug> Stack<T> where T: Clone {
    fn new()->Stack<T>{
        Stack {
            //head: Option::None,
            head: ptr::null_mut(),
        }
    }

    fn push(&mut self, elem: T){
        //println!("head_1={:?}", self.head);
        //let ptr_move = self.head.clone();

        let mut new_node = Node{
            elem: elem,
            //next: ptr::null_mut(),
            next: self.head,
        } ;
        println!("new_node = {:?}",new_node);

        //new_node.next = self.head;
        //let ptr_node: *mut _ = &mut new_node as *mut _;
        let ptr_node: *mut _ = Box::into_raw(Box::new(new_node));
        //println!("new_node = {:?}, ptr of new_node = {:?}",new_node, ptr_node);
        //println!("ptr_node={:?}", ptr_node);
        self.head = ptr_node;
        //unsafe{
        //    drop_in_place(ptr_node);
        //    dealloc(ptr_node as *mut u8, Layout::new::<Node<T>>());
        //}
    }
}

fn main() {
    let mut stack: Stack<i32> = Stack::new();
    //let v = vec![2,3,5,8];
    //for i in &v{
    //    stack.push(*i);
    //    //println!("i={:?}", i);  
    //}
   
    for i in 1..100 {
        stack.push(i);
    }

    let mut ptr_move: *mut _ = stack.head;
    let mut counter = 0;
    while !ptr_move.is_null()   {
        unsafe{
            //println!("node = {:?}", *(*ptr_move).next);
            println!("elem = {:?}", (*ptr_move).elem);
            ptr_move = (*ptr_move).next;
        }
        counter += 1;
    }
    println!("{:?}", stack);


}
