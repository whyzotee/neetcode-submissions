impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();
        
        loop {
            if l >= r {
                break;
            }
            
            let mid = (l+r)/2;
            if nums[mid] > target {
                r -= 1;
            } else if nums[mid] < target {
                l += 1;
            } else {
                return mid as i32;
            }
        }

        -1
    }
}
