//结构：
//[ptr]->[elem, ptr]->[null]

use std::ptr;
use std::{
    alloc::{
        dealloc, 
        Layout},
    ptr::drop_in_place,
};

#[derive(Debug)]
pub struct Stack<T> {
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
    pub fn new()->Self{
        Stack {
            //head: Option::None,
            head: ptr::null_mut(),
        }
    }

    pub fn is_empty(&self)->bool{
        if self.head.is_null() {
            true
        }else{
            false
        }
    }

    pub fn push(&mut self, elem: T){
        //println!("head_1={:?}", self.head);
        //let ptr_move = self.head.clone();

        let new_node = Node{
            elem: elem,
            //next: ptr::null_mut(),
            next: self.head,
        } ;
        //println!("new_node = {:?}",new_node);

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

    pub fn pop(&mut self)->Option<T> {
        if self.head.is_null() {
            Option::None
        }else{
            let ptr_result: *mut _ = self.head;
            unsafe{
                //let ptr_result: *mut _ = Box::into_raw(Box::new(*self.head));
                //(*self.head).next = ptr_result;
                self.head = (*self.head).next;
                //println!("ptr_result = {:?}", (*ptr_result).elem);
                Option::Some((*ptr_result).elem.clone())
            }
        }
        //unimplemented!();
        //Option::None
    }

    pub fn iter(&self) {
        let mut ptr_move: *mut _ = self.head;
        while !ptr_move.is_null() {
            unsafe{
                println!("{:?}", (*ptr_move).elem);
                ptr_move = (*ptr_move).next;
            }
        }
    }
}
