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
    depth: u32,
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
        let mut left = Self::null_new();
        let mut right = Self::null_new();
        let mut node = Node{
            //data: Some(_data),
            data: &_data as *const i32,
            parent: ptr::null_mut(),
            left: &mut left as *mut Node,
            right: &mut right as *mut Node,
            depth: left.depth - right.depth,
        };
        left.parent = &mut node as *mut Node;
        right.parent = &mut node as *mut Node;
        node
    }
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

   pub fn search(&self,_data: i32)->Option<*mut Node>{
        let mut ptr_mut: *mut _ = self.root;
        if ptr_mut.is_null() {
            return None
        }else{
            unsafe{
                while !(*ptr_mut).data.is_null(){
                    if (*(*ptr_mut).data) == _data {
                        break;
                    }
                }
            }
        }
        Option::Some(ptr_mut)
        //unimplemented!();
   }

   fn rotate(&self, _node: Node)->(){
       //左右孩子节点深度相差超过2， 进行旋 转
        
        if _node.depth < -1  {//左旋    
            
            let ptr_mut: *mut Node = _node.right;
            unsafe{
                while !(*ptr_mut).data.is_null(){
                    ptr_mut = (*ptr_mut).left;
                }

            }
        }else if _node.depth > 1{//右旋 

        }

        unimplemented!();
   }

   pub fn insert(&self,_data: i32)->(){
        unimplemented!();
   }

   pub fn delete(&self, _data: i32)->(){
        unimplemented!();
   }
}


