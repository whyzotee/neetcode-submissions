impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = (nums.len() - 1) as i32;
        
        loop {
            if l > r {
                break;
            }
            
            let m = l + (r - l) / 2;
            if nums[m as usize] > target {
                r = (m - 1);
            } else if nums[m as usize] < target {
                l = (m + 1);
            } else {
                return m;
            }
        }

        -1
    }
}
