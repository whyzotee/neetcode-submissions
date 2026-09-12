impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut nums = nums;
        nums.sort();

        let mut max_count_sequence = 1;
        let mut count_sequence = 1;

        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                continue;
            }

            if nums[i] + 1 == nums[i + 1] {
                count_sequence += 1;
            } else {
                count_sequence = 1;
            }

            if count_sequence > max_count_sequence {
                max_count_sequence = count_sequence;
            }
        }

        max_count_sequence
    }
}
