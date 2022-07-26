//布局： [ptr]->[val, next, down]->[val, next, down]->[val, next, down]->[val, null, null]
use std::ptr;
use rand::{prelude::*, Rng};
//skiplist是一个动态数组， 记录所有索引的头结构
//node_list是记录索引的数组
//node-total记录节点 总数
#[derive(Debug)]
pub struct SkipList{
    node_list: Vec<LinkedList>,
    node_total: u32,
}

//每层索引的链表，记录头指针和每层节点数
#[derive(Debug)]
#[derive(Clone)]
struct LinkedList {
    head: *mut Node,
    node_per_lev: u32,
}

//val: 数据
//next： 指向同一层的下一个节点
//down： 指向下一层的同一个节点
#[derive(Debug)]
#[derive(Clone)]
struct Node{
    //idx: i32,
    val: i32,
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

impl LinkedList{
    fn new(mut node: Node)->Self{
        LinkedList{
            head: &mut node as *mut Node,
            node_per_lev: 1,
        }
    }
}

impl SkipList {
    fn new()->Self{
        SkipList{
            node_list: Vec::new(),
            node_total: 0,
        }
    }
    //搜索节点，若存在则返回
    fn search(&self, _val: i32) -> Option<Node> {
        unimplemented!();
    }

    fn insert(&mut self, _val: i32 ) {
        let new_node = Node::new(_val);
        //let mut rng = rand::thread_rng();
        //let rand_lev = rng.gen_range();
        //println!("range={}", rng.gen_range(1..6));
        if self.node_list.is_empty() {
            self.node_list.push(LinkedList::new(Node::new(_val)));
        }
    }

    fn delete(&self, _val: i32) ->  Option<i32>{
        unimplemented!();
    }
    
}

fn main() {
    //let sl = SkipList::new();
    //let node = sl.search(87);
    //println!("node = {:?}", node);
    //
    //println!("char={}", rand::random::<u32>());
    let mut rng = rand::thread_rng();
    println!("range={}", rng.gen_range(1..6));
    //if rand::random(){
    //    println!("char={}", rand::random::<u32>());
    //}
}
