use std::ptr;
use std::option::Option;
struct Node{
    //parent: *mut Node,
    //data: Option<i32>,
    //data: *mut i32,
    data: *const i32,
    //link: Option<Link>,
    //this: *mut Node,
    parent: *mut Node,
    left: *mut Node,
    right: *mut Node,
    depth: i32,
}

//struct Link{
//    //data: i32,
//    this: *mut Node,
//    parent: *mut Node,
//    left: *mut Node,
//    right: *mut Node,
//    //depth: u32,
//}

struct AvlTree{
    root: *mut Node,
    //root: Link,
}


impl Node{
    fn null_new()->Self{
        Node{
            //data: None,
            data: ptr::null_mut(),
            parent: ptr::null_mut(),
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            depth: 0,
        }
    }

    fn new(_data: i32)->Self{
        Node {
            data: &_data as *const i32,
            parent: ptr::null_mut(),
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            depth: 0,
        }
    }

    //fn new(_data: i32)->Self{
    //    let mut left = Self::null_new();
    //    let mut right = Self::null_new();
    //    let mut node = Node{
    //        //data: Some(_data),
    //        data: &_data as *const i32,
    //        parent: ptr::null_mut(),
    //        left: &mut left as *mut Node,
    //        right: &mut right as *mut Node,
    //        depth: left.depth - right.depth,
    //    };
    //    left.parent = &mut node as *mut Node;
    //    right.parent = &mut node as *mut Node;
    //    node
    //}
}

//impl Link{
//    fn new(){
//        parent: ptr::null_mut(),
//        this: ptr::null_mut(),
//        left: ptr::null_mut(),
//        right: ptr::null_mut(),
//    }
//}

impl AvlTree {
   pub fn new()->Self{
        AvlTree{
            root: ptr::null_mut(),
        }
    }
   

    //pub fn new(_data: i32)->Self{
    //    let mut left = Self::null_new();
    //    let mut right = Self::null_new();
    //    let mut node = Node{
    //        //data: Some(_data),
    //        data: &_data as *const i32,
    //        parent: ptr::null_mut(),
    //        left: &mut left as *mut Node,
    //        right: &mut right as *mut Node,
    //        depth: left.depth - right.depth,
    //    };
    //    left.parent = &mut node as *mut Node;
    //    right.parent = &mut node as *mut Node;
    //    node
    //}

   //Node没有空值，所以不用Option<Node>,直接返回Node
   pub fn search(&self,_data: i32)->*mut Node{
        let mut ptr_mut: *mut _ = self.root;
        if ptr_mut.is_null() {
            ptr::null_mut()
        }else{
            let mut ptr_mut: *mut _ = self.root;
            unsafe{
                while !(*ptr_mut).data.is_null(){
                    if (*(*ptr_mut).data) == _data {
                        break;
                    }
                }
            }
            ptr_mut
        }
        //unimplemented!();
   }

    fn iterCaculateDepth(&self, _node: &mut Node) -> i32{
        if !_node.data.is_null(){
            //let ptr_left = _node.left;
            //let ptr_right = _node.right;
            unsafe{
                _node.depth = self.iterCaculateDepth(&mut (*_node.left))- self.iterCaculateDepth(&mut (*_node.right));
            }
        }
        _node.depth
    }

   fn rotate(&mut self, _ptr_node: *mut Node)->(){
       //左右孩子节点深度相差超过2， 进行旋 转
       unsafe{
            if (*_ptr_node).depth < -1  {//左旋    
                    let ptr_right: *mut Node = (*_ptr_node).right;
                    (*((*ptr_right).left)).parent = _ptr_node;
                    (*_ptr_node).right = (*ptr_right).left;
                    (*ptr_right).left =  _ptr_node;
                    (*_ptr_node).parent = ptr_right;
                    self.iterCaculateDepth(&mut (*ptr_right));
            }else if (*_ptr_node).depth > 1{//右旋 
                    let ptr_left: *mut Node = (*_ptr_node).left;
                    (*((*ptr_left).right)).parent = _ptr_node;
                    (*_ptr_node).left = (*ptr_left).right;
                    (*ptr_left).right =  _ptr_node;
                    (*_ptr_node).parent = ptr_left;
                    self.iterCaculateDepth(&mut (*ptr_left));
            }
       }
        //unimplemented!();
   }

   pub fn insert(&mut self,_data: i32)->(){
        if self.root.is_null(){
            let mut root = Node::new(_data);
            let mut left = Node::null_new();
            let mut right = Node::null_new();
            unsafe{
                root.left = &mut left as *mut _;
                root.right = &mut right as *mut _;
                left.parent = &mut root as *mut _;
                right.parent = &mut root as *mut  _;
            }
        }else{
            let ptr_node = self.search(_data);
            unsafe{
                if (*ptr_node).data.is_null() {
                    let ptr_parent: *mut _ = (*ptr_node).parent;
                    let mut new_node = Node::new(_data);
                    if *(*ptr_parent).data < _data {//新插入的节点小于当前节点
                        new_node.parent = ptr_parent;
                        (*ptr_node).parent = &mut new_node as *mut _;
                        new_node.right = ptr_node;
                        (*ptr_parent).right = &mut new_node as *mut _;
                        //add a new null left node
                        let mut new_left = Node::null_new(); 
                        new_left.parent = &mut new_node as *mut _;
                        new_node.left = &mut new_left as *mut _;
                    }else if *(*ptr_parent).data > _data {
                        new_node.parent = ptr_parent;
                        (*ptr_node).parent = &mut new_node as *mut _;
                        new_node.left = ptr_node;
                        (*ptr_parent).left = &mut new_node as *mut _;
                        //add a new null left node
                        let mut new_right = Node::null_new(); 
                        new_right.parent = &mut new_node as *mut _;
                        new_node.right = &mut new_right as *mut _;
                    }
                    let mut ptr_mut: *mut _ = new_node.parent;
                    while !(*ptr_mut).parent.is_null() {//reach the root , break the loop
                        if (*ptr_mut).depth <= -2 || (*ptr_mut).depth >= 2 {
                            self.rotate(ptr_mut);
                            self.iterCaculateDepth(&mut (*ptr_mut)) ;
                            break;
                        }
                        ptr_mut = (*ptr_mut).parent;
                    }
                }             
            }
        }
        //unimplemented!();
   }

   pub fn delete(&self, _data: i32)->(){
        unimplemented!();
   }
}


