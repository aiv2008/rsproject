//布局： [ptr]->[val, next, down]->[val, next, down]->[val, next, down]->[val, null, null]
//
//注意事项
//1. 需要一个数组，用来定位插入新元素的位置
//2. 每个节点是否需要一个向前的指针，用来追溯之前的元素，一边插入
use std::ptr;
use rand::{prelude::*, Rng};
use std::option::Option;
//mod unsafe_queue;
use std::cmp;

#[derive(Debug)]
pub struct SkipList{
    //head: Vec<*mut Node>,
    head: *mut Node,
    height: u32,
}

#[derive(Debug)]
#[derive(Clone)]
struct Node{
    val: i32,
    next: *mut Node,
    pre: *mut Node,
    down: *mut Node,
}


impl Node{
    fn new(val: i32)->Self{
        Node{
            val: val,
            next: ptr::null_mut(),
            pre: ptr::null_mut(),
            down: ptr::null_mut(),
        }
    }
}

impl SkipList {
    pub fn new()->Self{
        let mut ptr_min: *mut _ = Node::new(std::i32::MIN);
        let mut ptr_max: *mut _ = Node::new(std::i32::MAX);
        ptr_min.next = ptr_max;
        ptr_max.pre = ptr_min;
        SkipList{
            head: ptr_min,
            height: 0,
        }
       //unimplemented!();
    }

    //pub fn len(&self)->usize{
    //    self.head.len()
    //}

    //搜索节点，若存在则返回
    pub fn search(&self, _val: i32) -> bool {
        let mut bl_result = false;
        let mut ptr_mut: *mut _ = self.head;
        while !ptr_mut.is_null() {
            unsafe{
                if _val == (*ptr_mut).val{
                    bl_result = true;
                    break;
                }else{
                    if _val < (*(*ptr_mut).next).val {
                        ptr_mut = ptr_mut.down;
                        continue;
                    }
                }
            }
            ptr_mut = ptr_mut.next;
        }
        bl_result
        //unimplemented!();
    }

    pub fn iter(&self){
        //for ptr_node in &self.head{
        //vector倒序输出    
        for i in 0..self.head.len() {
            println!("--begin---");
            //let mut ptr_mut = *ptr_node;
            let mut ptr_mut = self.head[self.head.len()-1-i];
            while !ptr_mut.is_null() {
                unsafe{
                    //遇到头节点和尾节点不访问
                    if true || ( (*ptr_mut).val != std::i32::MAX &&  (*ptr_mut).val != std::i32::MIN) {
                        print!("{}: {:?}, ", (*ptr_mut).val, ptr_mut);
                        if !(*ptr_mut).next.is_null(){
                            print!("{}: {:?}, ",(*(*ptr_mut).next).val, (*ptr_mut).next);
                        }else{
                            print!("null, ");
                        }
                        if !(*ptr_mut).down.is_null(){
                            print!("{}: {:?}, ",(*(*ptr_mut).down).val, (*ptr_mut).down);
                        }else{
                            print!("null, ");
                        }
                        println!("");
                    }
                    ptr_mut = (*ptr_mut).next;
                }
            }
        }
    }

    //前后各加入一个节点，分别是最小值和最大值，方便比较插入
    pub fn insert(&mut self, _val: i32 ) {
        let mut new_node = Node::new(_val);
        if self.head.is_empty() {
            let mut new_n_node = Node::new_n_inf();
            let  new_p_node = Node::new_p_inf();
            new_n_node.next = Box::into_raw(Box::new(new_node.clone())) ;
            unsafe{
                (*new_n_node.next).next = Box::into_raw(Box::new(new_p_node.clone())) ;
            }
            self.head.push(Box::into_raw(Box::new(new_n_node.clone())));
        }else {
            //let mut ptr_move: *mut _ = self.head[self.head.len()-1];
            if !self.search(_val) {//若找不到元素，则插入
                
            }
        //unimplemented!();
    }

    pub fn delete(&self, _val: i32) ->  Option<i32>{
        unimplemented!();
    }
    
}

//fn main() {
//    let mut rng = rand::thread_rng();
//    println!("range={}", rng.gen_range(1..6));
//}
