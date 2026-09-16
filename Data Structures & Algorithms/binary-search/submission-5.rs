impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() as i32;
        
        loop {
            if r <= l {
                break;
            }
            
            let m = (l + (r - l) / 2) as usize;
            if nums[m] > target {
                r -= 1;
            } else if nums[m] < target {
                l += 1;
            } else {
                return m as i32;
            }
        }

        -1
    }
}
