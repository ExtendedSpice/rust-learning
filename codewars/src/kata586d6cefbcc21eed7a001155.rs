// https://www.codewars.com/kata/586d6cefbcc21eed7a001155

pub fn longest_repetition(s: &str) -> Option<(char, usize)> {
    if s.len() == 0 {
        return None;
    }

    let mut result_char = ' ';
    let mut result_count: usize = 0;

    let mut cursor_char = result_char;
    let mut cursor_count = result_count;

    for char in s.chars() {
        if char == cursor_char {
            cursor_count += 1;
        } else {
            cursor_char = char;
            cursor_count = 1;
        }
        if cursor_count > result_count {
            result_char = cursor_char;
            result_count = cursor_count;
        }
    }

    Some((result_char, result_count))
}
