use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
        for w in strs {
            let mut key = w.as_bytes().to_vec();
            key.sort();
            map.entry(key).or_insert(vec![]).push(String::from(w));
        }

        let mut res = Vec::new();

        for v in map.values() {
            res.push(v.clone());
        }

        res
    }
}
