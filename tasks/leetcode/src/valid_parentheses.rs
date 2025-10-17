pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }

        let mut stack: Vec<char> = Vec::new();

        for elem in s.chars() {
            match elem {
                '(' | '{' | '[' => stack.push(elem),
                ')' => {
                    if Self::is_not_match_pattern(&mut stack, '(') {
                        return false;
                    }
                }
                '}' => {
                    if Self::is_not_match_pattern(&mut stack, '{') {
                        return false;
                    }
                }
                ']' => {
                    if Self::is_not_match_pattern(&mut stack, '[') {
                        return false;
                    }
                }
                _ => {
                    continue;
                }
            }
        }

        stack.is_empty()
    }

    fn is_not_match_pattern(stack: &mut Vec<char>, pattern: char) -> bool {
        if let Some(e) = stack.pop() {
            e != pattern
        } else {
            false
        }
    }
}
