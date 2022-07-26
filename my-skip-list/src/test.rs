use rand::prelude::*;

#[test]
fn test_ram(){
    if rand::random(){
        println!("char={}",rand::random::<char>());
    }
}
