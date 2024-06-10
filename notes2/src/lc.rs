struct Solution {}

// 3181. 执行操作可获得的最大总奖励 II
// https://leetcode.cn/problems/maximum-total-reward-using-operations-ii/description/
// !非常关键的一点是：获取vec下标元素用get_unchecked_mut，这样可以避免越界检查，提高性能
impl Solution {
    pub fn max_total_reward(mut reward_values: Vec<i32>) -> i32 {
        unsafe { Self::max_total_reward_unsafe(reward_values) }
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn max_total_reward_unsafe(mut reward_values: Vec<i32>) -> i32 {
        reward_values.sort_unstable();
        reward_values.dedup();
        let max = reward_values[reward_values.len() - 1];
        let mut dp = vec![false; 2 * max as usize + 1];
        dp[0] = true;

        for v in reward_values.into_iter() {
            for x in (v..(v << 1)).rev() {
                unsafe {
                    *(dp.get_unchecked_mut(x as usize)) |= *dp.get_unchecked((x - v) as usize);
                }
            }
        }

        dp.iter().enumerate().rfind(|(_, &x)| x).unwrap().0 as i32
    }
}

// 2935. 找出强数对的最大异或值 II
// https://leetcode.cn/problems/maximum-strong-pair-xor-ii/description/
impl Solution {
    pub fn maximum_strong_pair_xor(mut nums: Vec<i32>) -> i32 {
        let mut res = 0;
        nums.sort_unstable();
        for (i, v1) in nums.iter().enumerate() {
            for v2 in nums.iter().take(i).skip_while(|&v| 2 * v < *v1) {
                res = res.max(v1 ^ v2);
            }
        }

        res
    }
}
