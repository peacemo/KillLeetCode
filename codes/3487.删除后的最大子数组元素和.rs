/*
 * @lc app=leetcode.cn id=3487 lang=rust
 *
 * [3487] 删除后的最大子数组元素和
 */

// @lc code=start
impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut num_set = HashSet::new();
        let mut sum =0; 
        let mut max_sum = i32::MIN;

        for n in nums {
            if n < 0 {
                max_sum = max_sum.max(n);
            } else if num_set.insert(n) {
                sum += n;
            }
        }
        
        if num_set.is_empty() {
            max_sum
        } else {
            sum
        }
    }
}
// @lc code=end

