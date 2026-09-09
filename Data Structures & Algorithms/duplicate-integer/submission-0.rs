impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        let mut temp = Vec::new();

        while let Some(num) = nums.pop() {
            if temp.contains(&num) {
                return true;
            }

            temp.push(num);
        }

        false
    }
}
