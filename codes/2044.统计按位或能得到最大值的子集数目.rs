/*
 * @lc app=leetcode.cn id=2044 lang=rust
 *
 * [2044] 统计按位或能得到最大值的子集数目
 */

// @lc code=start
impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut max_val = i32::MIN;
        let mut result = 0;
        let n = nums.len();

        for i in 0..(1 << n) {
            let mut or_val = 0;
            for j in 0..n {
                if (i >> j) & 1 == 1 {
                    or_val |= nums[j];
                }
            }

            if max_val < or_val {
                max_val = or_val;
                result = 1;
            } else if max_val == or_val{
                result += 1;
            }
        }

        result
    }

    pub fn calculate_bit_or(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];

        for i in (1..nums.len()) {
            result = result | nums[i];
        }

        result
    }
}
// @lc code=end

