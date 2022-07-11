//devuse std::rc::Rc;
use std::ptr;
#[derive(Debug)]
#[derive(Clone)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

//type Link<T> = Option<*mut Node<T>>;

#[derive(Debug)]
struct MyQueue<T>{
    head: Link<T>,
    //tail: Link<T>,
    //tail: Option<*mut Node<T>>,
    tail: *mut Node<T>,
}

impl <T: std::fmt::Debug> MyQueue<T> where T: Clone{
    fn new()->Self {
        MyQueue{
            head: Option::None,
            //tail: Option::None,
            tail: ptr::null_mut(),
        }
    }

    fn push(&mut self, elem: T) {
        //let mut node = Node{
        //    elem: elem,
        //    next: Link::None,
        //};

        let mut box_node = Box::new(Node{
            elem: elem,
            next: Link::None,
        });

        let raw_tail: *mut _ = &mut *box_node;

        if self.tail.is_null(){
            self.head = Link::Some(box_node);
        }else{
            unsafe{
                (*self.tail).next = Link::Some(box_node);        
            }
        }
        //self.tail  = &mut *box_node as *mut Node<T>; 
        self.tail = raw_tail;
    }

    fn pop(&mut self)->Option<T>{
        //match &mut self.head {
        match std::mem::replace(&mut self.head, Option::None) {
            Option::Some(x)=>{
                let ret = x.clone();
                //println!("x.next is {:?}", x.next);
                //assert_eq!(x, 5);
                self.head = x.next;
                Option::Some(ret.elem)
            },
            _=>{
                self.tail = ptr::null_mut();
                Option::None
            },
        }
        //if self.tail.is_null() {//tail is none ,return none
        //    Option::None
        //}else{
        //    unsafe{
        //        let tail_val = &*self.tail;
        //        match &mut (*self.tail).next {
        //            Option::Some(x)=>{//x can be deref as node
        //                //assert_eq!(**x, Box::new(Option::None));
        //                self.tail = &mut **x;
        //            },
        //            _=>{
        //                self.tail = ptr::null_mut();
        //            },
        //        }
        //        Option::Some((*tail_val).elem.clone())
        //    }
        //}
        //unimplemented!();    
    }
}

fn main() {
    let mut my_queue = MyQueue::new();
    println!("head={:?},tail={:?}", my_queue.head, my_queue.tail);
    println!("{:?}", my_queue);
    let v = vec![1,2,3,4,5];
    for i in &v{
        my_queue.push(i);
    }
    //my_queue.push(23);
    //my_queue.push(323);
    println!("head={:?},tail={:?}", my_queue.head, my_queue.tail);
    println!("{:?}", my_queue);
    for _ in &v {
        println!("{:?}", my_queue.pop()) ;
    }
    println!("{:?}", my_queue.pop()) ;
    println!("head={:?},tail={:?}", my_queue.head, my_queue.tail);
    println!("{:?}", my_queue);
}
