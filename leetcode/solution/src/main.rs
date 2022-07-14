fn main() {
    let v = vec![1, 2, 3];
    let result = Solution::pivot_index(v);
    println!("{}", result);
}

struct Solution();

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut mid_index = nums.len()/2;
        //let mut result: i32 = 0;
        let mut left_sum = 0;
        let mut right_sum = 0;
        let mut b_comp: i32 = 0;//左右和比较， 左大返回-1，右大返回1，相等返回0
        for i in &nums[..mid_index]{
            left_sum += *i;    
        }
        for i in &nums[(mid_index+1)..]{
            right_sum += *i;    
        }
        b_comp = {
            if left_sum > right_sum {
                -1
            }else{
                if left_sum < right_sum {
                    1
                }else{
                    0
                }
            }
        };
        println!("mid={}, left={}, right={}", mid_index, left_sum, right_sum);
        //(left_sum, right_sum) = (0, 0);
        //let tmp_index = 0;
        while b_comp != 0 {
            if b_comp == -1{

                mid_index-=1; 
            }else {
                mid_index+=1;
            }
            (left_sum, right_sum) = (0, 0);
            for i in &nums[..mid_index]{
                left_sum += *i;    
            }
            for i in &nums[(mid_index+1)..]{
                right_sum += *i;    
            }
            println!("left_sum={}, right_sum={}", left_sum, right_sum);
            b_comp = {
                if left_sum > right_sum {
                    -1
                }else{
                    if left_sum < right_sum {
                        1
                    }else{
                        0
                   }
                }
            };
            //tmp_index = mid_index;
        }

        println!("mid_index={}, left={}, right={}", mid_index, left_sum, right_sum);
        if left_sum == right_sum {
             mid_index as i32
        }else{
           -1 
        }
        //mid_index
        //unimplemented!();
    }

}
