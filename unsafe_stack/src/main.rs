//use crate::Stack;
mod unsafe_stack;
fn main(){
    let stack = unsafe_stack::Stack::<i32>::new();
}
