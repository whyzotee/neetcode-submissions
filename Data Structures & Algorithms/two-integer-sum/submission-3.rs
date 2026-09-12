use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut diff: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            let diff_num = target - nums[i];

            if diff.contains_key(&diff_num) {
                let j = diff.get(&diff_num).unwrap().to_owned();
                return vec![j, i as i32];
            }

            diff.insert(nums[i], i as i32);
        }

        vec![]
    }
}
