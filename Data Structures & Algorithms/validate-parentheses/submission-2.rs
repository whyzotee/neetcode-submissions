impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut s = s.chars();
        let mut stack = Vec::new();

        while let Some(c) = s.next() {
            if c == '(' || c == '[' || c == '{' {
                stack.push(c);
            }

            if c == ')' {
                let open_char = stack.pop();

                if open_char.is_none() {
                    return false;
                }

                if let Some(close) = open_char {
                    if close != '(' {
                        return false;
                    }
                }
            }

            if c == ']' {
                let open_char = stack.pop();

                if open_char.is_none() {
                    return false;
                }

                if let Some(close) = open_char {
                    if close != '[' {
                        return false;
                    }
                }
            }

            if c == '}' {
                let open_char = stack.pop();

                if open_char.is_none() {
                    return false;
                }

                if let Some(close) = open_char {
                    if close != '{' {
                        return false;
                    }
                }
            }
        }

        stack.len() == 0
    }
}
