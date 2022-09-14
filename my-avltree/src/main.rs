mod my_avl;
fn main() {
	let v: Vec<i32> = vec![2,1,3,2,1];
	let  mut tree = my_avl::AvlTree::new();
	for e in v{
		tree.insert(e);
	}

	tree.iter();
    println!("Hello, world!");
}
