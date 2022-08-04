//devuse std::rc::Rc;
use std::ptr;
#[derive(Debug)]
#[derive(Clone)]
struct Node<T> {
    elem: T,
    next: Link<T>,
    //next: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;


impl <T> Node<T> {
    fn new(elem: T )->Self{
        Node{
            elem: elem,
            next: Option::None,
        }
    }
}
#[derive(Debug)]
pub struct MyQueue<T>{
    head: Link<T>,
    tail: *mut Node<T>,
}

impl <T: std::fmt::Debug> MyQueue<T> where T: Clone{
    pub fn new()->Self {
        MyQueue{
            head: Option::None,
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {

        //let mut box_node = Box::new(Node{
        //    elem: elem,
        //    next: Link::None,
        //});

        let mut box_node = Box::new(Node::new(elem));
        let raw_tail: *mut _ = &mut *box_node;
        //println!("raw_tail={:?}", raw_tail);

        if self.tail.is_null(){
            self.head = Link::Some(box_node);
        }else{
            unsafe{
                (*self.tail).next = Link::Some(box_node);        
            }
        }
        self.tail = raw_tail;
    }

    pub fn pop(&mut self)->Option<T>{
        match std::mem::replace(&mut self.head, Option::None) {
            Option::Some(x)=>{
                let ret = x.clone();
                self.head = x.next;
                Option::Some(ret.elem)
            },
            _=>{
                self.tail = ptr::null_mut();
                Option::None
            },
        }
        //unimplemented!();
    }
}

