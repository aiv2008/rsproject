fn main(){
    let v = vec![3,1,2,10,1];
    let solution = Solution();
    let v2 =Solution::running_sum(v);
    println!("{:?}", v2);
}


struct Solution();


impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut r_vec: Vec<i32> = Vec::new();
        let mut tmp = 0;
        for i in &nums {
            tmp += *i;
            r_vec.push(tmp);
        }
        r_vec
        //unimplemented!();
    }
}
