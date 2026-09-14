impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut encoded_string = String::new();

        for (i, word) in strs.iter().enumerate() {
            if word.is_empty() {
                encoded_string.push(',');
                continue;
            }

            let bytes = word.as_bytes();

            for c in bytes {
                encoded_string.push_str(&c.to_string());
                encoded_string.push('#');
            }

            encoded_string.push(',');
        }

        encoded_string
    }

    pub fn decode(s: String) -> Vec<String> {
        let split_text: Vec<&str> = s.split(',').collect();
        let mut res = Vec::new();

        for s in split_text {
            let mut word = String::new();

            for bytes in s.split('#') {
                if bytes.is_empty() {
                    continue;
                }

                word.push(char::from(bytes.parse::<u8>().unwrap()));
            }

            res.push(word);
        }

        res.pop();
        res
    }
}
