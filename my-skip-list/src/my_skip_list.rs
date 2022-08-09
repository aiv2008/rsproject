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
    next: *mut Node,
    pre: *mut Node,
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

    fn new_n_inf()->Self{
        Node{
            val: std::i32::MIN,
            next: ptr::null_mut(),
            down: ptr::null_mut(),
        }
    }

    fn new_p_inf()->Self{
        Node{
            val: std::i32::MAX,
            next: ptr::null_mut(),
            down: ptr::null_mut(),
        }
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

    pub fn len(&self)->usize{
        self.head.len()
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
                    //println!("ptr_mut.is_null: {}",  ptr_mut.is_null());
                    ptr_mut = (*ptr_mut).next;
                }
                
            }
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
            //new_node.next = Box::into_raw(Box::new(new_p_node.clone())) ;
            unsafe{
                (*new_n_node.next).next = Box::into_raw(Box::new(new_p_node.clone())) ;
            }
            self.head.push(Box::into_raw(Box::new(new_n_node.clone())));
            //self.head.push(Box::into_raw(Box::new(new_node.clone())));
        }else {
            //let mut ptr_move: *mut _ = self.head[self.head.len()-1];
            if !self.search(_val) {//若找不到元素，则插入
                let rand_pos =  (rand::thread_rng()).gen_range(1..(2*self.head.len()+1));
                let pos = cmp::min(rand_pos, self.head.len());
                let mut i:i32 = (pos as i32) -1;
                let mut ptr_mut: *mut Node = ptr::null_mut();
                let mut ptr_start: *mut Node = ptr::null_mut();
                while i >=0 {//从随机生成的第i层开始插入
                    //ptr_start = self.head[i as usize];
                    ptr_mut = self.head[i as usize];
                    //ptr_mut = ptr_start;
                    unsafe{
                        //ptr_start取第一个节点（负无穷大）后的那个节点
                        ptr_start =(*ptr_mut).next; 
                        while !(*ptr_mut).next.is_null(){
                            if _val < (*(*ptr_mut).next).val{//若输入的节点小于当前节点的下一个节点next，则在当前节点与下个节点next之间插入该输入节点
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
                            println!("(*ptr_mut).val is {}", (*ptr_mut).val);
                           let mut new_node_raw: *mut _ = Box::into_raw(Box::new(new_node.clone()));
                           (*ptr_mut).next = new_node_raw;
                           while !(*ptr_mut).down.is_null(){
                                ptr_mut = (*ptr_mut).down;
                                let mut new_node_raw: *mut _ = Box::into_raw(Box::new(new_node.clone()));
                                (*ptr_mut).next = new_node_raw;
                           }
                        }
                    }
                    i-=1;
                }

                let max_ele = cmp::max(rand_pos,self.head.len()); 
                if max_ele> self.head.len() {
                    let self_head_len = self.head.len();
                    for _ in self_head_len..max_ele {
                        let mut new_n_node = Node::new_n_inf();
                        let  new_p_node = Node::new_p_inf();
                        new_n_node.next = Box::into_raw(Box::new(new_node.clone())) ;
                        unsafe{
                            (*new_n_node.next).next = Box::into_raw(Box::new(new_p_node.clone())) ;
                            println!("ptr_start={:?}", ptr_start);
                            (*new_n_node.next).down = ptr_start; 
                        }
                        self.head.push(Box::into_raw(Box::new(new_n_node.clone())));
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
