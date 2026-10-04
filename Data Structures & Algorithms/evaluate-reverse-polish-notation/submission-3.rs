impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        
        for n in tokens {
            match n.as_str() {
                "+" => {
                    let x = stack.pop().unwrap(); 
                    let y = stack.pop().unwrap();
                    
                    let sum = x+y;
                    stack.push(sum);
                },
                "-" => {
                    let x = stack.pop().unwrap(); 
                    let y = stack.pop().unwrap();
                    
                    let sum = y-x;
                    stack.push(sum);
                },
                "*" => {
                    let x = stack.pop().unwrap(); 
                    let y = stack.pop().unwrap();
                    
                    let sum = x*y;
                    stack.push(sum);
                },
                "/" => {
                    let x = stack.pop().unwrap(); 
                    let y = stack.pop().unwrap();
                    
                    let sum = y/x;
                    stack.push(sum);
                },
                _ => stack.push(n.parse::<i32>().unwrap())
            };
        }

        stack[0]
    }
}
