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
    //vector是一个盏结构
    head: Vec<*mut Node>,
}

#[derive(Debug)]
#[derive(Clone)]
struct Node{
    val: i32,
    //link: Link,
    next: *mut Node,
    down: *mut Node,
}


impl Node{
    fn new(val: i32)->Self{
        Node{
            val: val,
            next: ptr::null_mut(),
            down: ptr::null_mut(),
        }
        //unimplemented!();
    }
}

impl SkipList {
    pub fn new()->Self{
       //SkipList::More(Link::new())
       SkipList{
            head: Vec::new(), 
       }
       //unimplemented!();
    }
    //搜索节点，若存在则返回
    pub fn search(&self, _val: i32) -> bool {
        let mut bl_result = false;
        if  !self.head.is_empty(){
            let mut ptr_mut: *mut _ = self.head[self.head.len()-1];
            unsafe{
                while !ptr_mut.is_null() && !(*ptr_mut).next.is_null() {
                    if _val == (*(*ptr_mut).next).val {
                        //ptr_mut = (*ptr_mut).next;
                        bl_result = true;
                        break;
                    }else if _val < (*(*ptr_mut).next).val {
                        ptr_mut = (*ptr_mut).down;
                        if ptr_mut.is_null() {
                            break;
                        }
                    }
                    println!("ptr_mut.is_null: {}",  ptr_mut.is_null());
                    ptr_mut = (*ptr_mut).next;
                }
                
            }
            //if ptr_mut.is_null(){
            //    return false;
            //}else{
            //    return true;
            //}
        }
        bl_result
        //unimplemented!();
    }


    pub fn insert(&mut self, _val: i32 ) {
        let new_node = Node::new(_val);
        if self.head.is_empty() {
            self.head.push(Box::into_raw(Box::new(new_node.clone())));
        }else {
            //let mut ptr_move: *mut _ = self.head[self.head.len()-1];
            if !self.search(_val) {//若找不到元素，则插入
                let rand_pos =  (rand::thread_rng()).gen_range(1..(2*self.head.len()));
                //println!("range={}", rng.gen_range(1..6));
                //let new_node = Node::new(_val);
                let pos = cmp::min(rand_pos, self.head.len());
                let mut i:i32 = (pos as i32) -1;
                let mut ptr_mut: *mut Node = ptr::null_mut();
                let mut ptr_start: *mut Node = ptr::null_mut();
                //let mut new_node_raw: *mut _ = Box::into_raw(Box::new(new_node.clone()));
                while i >=0 {
                    ptr_start = self.head[i as usize];
                    //ptr_mut = self.head[i];
                    ptr_mut = ptr_start;
                    //let mut new_node_raw: *mut _ = Box::into_raw(Box::new(new_node.clone()));
                    //while !ptr_mut.is_null(){
                    
                    unsafe{
                        while !(*ptr_mut).next.is_null(){
                            if _val < (*(*ptr_mut).next).val{
                                let mut new_node_raw: *mut _ = Box::into_raw(Box::new(new_node.clone()));
                                (*new_node_raw).next = (*ptr_mut).next;
                                (*ptr_mut).next = new_node_raw;
                                if (*ptr_mut).down.is_null() {
                                    break;
                                }
                                ptr_mut = (*ptr_mut).down;
                                continue;
                            }
                            ptr_mut = (*ptr_mut).next;
                        }
                        if (*ptr_mut).next.is_null(){
                           let mut new_node_raw: *mut _ = Box::into_raw(Box::new(new_node.clone()));
                           (*ptr_mut).next = new_node_raw;
                           while !(*ptr_mut).down.is_null(){
                                ptr_mut = (*ptr_mut).down;
                                let mut new_node_raw: *mut _ = Box::into_raw(Box::new(new_node.clone()));
                                (*ptr_mut).next = new_node_raw;
                           }
                        }
                    }
                    println!("i={}",  i);
                    i-=1;
                }

                if rand_pos > self.head.len() {
                    for _ in rand_pos..self.head.len(){
                        let mut new_node_raw: *mut _ = Box::into_raw(Box::new(new_node.clone()));
                        unsafe{
                            (*new_node_raw).down = ptr_start;
                        }
                        self.head.push(new_node_raw);
                        ptr_start = self.head[self.head.len()-1];
                    }
                }
            }
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
