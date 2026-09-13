impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut most: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];
        let mut count: HashMap<i32, i32> = HashMap::new();

        for n in nums {
            *count.entry(n).or_insert(0) += 1;
        }

        for (n, c) in count.iter() {
            most[*c as usize].push(*n);
        }

        let mut res = Vec::new();

        for i in (0..most.len()).rev() {
            for n in most[i].iter() {
                res.push(*n);

                if res.len() as i32 == k {
                    return res;
                }
            }
        }

        res
    }
}
