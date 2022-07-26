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
struct MyQueue<T>{
    head: Link<T>,
    tail: *mut Node<T>,
}

impl <T: std::fmt::Debug> MyQueue<T> where T: Clone{
    fn new()->Self {
        MyQueue{
            head: Option::None,
            tail: ptr::null_mut(),
        }
    }

    fn push(&mut self, elem: T) {

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

    fn pop(&mut self)->Option<T>{
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

fn main() {
    let mut my_queue = MyQueue::<Node<i32>>::new();
    //println!("head={:?},tail={:?}", my_queue.head, my_queue.tail);
    //println!("{:?}", my_queue);

    let v = vec![1,2,3,4,5];
    for i in &v{
        let node = Node::new(*i);
        //my_queue.push(*i);
        my_queue.push(node);
    }
    //println!("head={:?},tail={:?}", my_queue.head, my_queue.tail);
    println!("{:?}", my_queue);
    //for _ in &v {
    //    println!("{:?}", my_queue.pop()) ;
    //}
    //println!("{:?}", my_queue.pop()) ;
    //println!("head={:?},tail={:?}", my_queue.head, my_queue.tail);
    //println!("{:?}", my_queue);
}
