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

   fn rotate(&mut self, mut _node: Node)->(){
       //左右孩子节点深度相差超过2， 进行旋 转
        if _node.depth < -1  {//左旋    
            let ptr_right: *mut Node = _node.right;
            unsafe{
                (*((*ptr_right).left)).parent = &mut _node as *mut _;
                _node.right = (*ptr_right).left;
                (*ptr_right).left = &mut _node as *mut _;
                _node.parent = ptr_right;
                self.iterCaculateDepth(&mut (*ptr_right));
            }
        }else if _node.depth > 1{//右旋 
            let ptr_left: *mut Node = _node.left;
            unsafe{
                (*((*ptr_left).right)).parent = &mut _node as *mut _;
                _node.left = (*ptr_left).right;
                (*ptr_left).right = &mut _node as *mut _;
                _node.parent = ptr_left;
                self.iterCaculateDepth(&mut (*ptr_left));
            }
        }

        unimplemented!();
   }

   pub fn insert(&self,_data: i32)->(){
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
                
            }
        }
        unimplemented!();
   }

   pub fn delete(&self, _data: i32)->(){
        unimplemented!();
   }
}


