impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![1; n];
        let mut pref = vec![1; n];
        let mut suff = vec![1; n];

        for i in 1..pref.len() {
            pref[i] = nums[i - 1] * pref[i - 1]
        }

        for i in (0..suff.len() - 1).rev() {
            suff[i] = nums[i + 1] * suff[i + 1];
        }

        for i in 0..res.len() {
            res[i] = pref[i] * suff[i]
        }

        res
    }
}
