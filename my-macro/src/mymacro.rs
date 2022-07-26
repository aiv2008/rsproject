#[macro_export]
macro_rules! my_vec{
    ( $($x:expr),* )=>{
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

mod test{
#[test]
fn test_macro(){
    let v = my_vec![1,5,6,7,8];  
    for i in &v{
        prnitln!("{}", i);
    }
    assert_eq!(1,2);
}
}
