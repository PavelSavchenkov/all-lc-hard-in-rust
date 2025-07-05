//! Solution for https://leetcode.com/problems/design-a-text-editor
//! 2296. Design a Text Editor

struct TextEditor {
    prev: Vec<Option<usize>>,
    next: Vec<Option<usize>>,
    chars: Vec<u8>,
    cursor: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TextEditor {
    fn new() -> Self {
        Self {
            prev: vec![None, Some(0)],
            next: vec![Some(1), None],
            chars: vec![b' ', b' '],
            cursor: 1,
        }
    }

    fn add_text(&mut self, text: String) {
        let text = to_u8(&text);
        let mut prev = self.prev[self.cursor].unwrap();
        for &c in &text {
            let i = self.chars.len();
            self.prev.push(Some(prev));
            self.next[prev] = Some(i);
            self.next.push(None);
            self.chars.push(c);
            prev = i;
        }
        self.next[prev] = Some(self.cursor);
        self.prev[self.cursor] = Some(prev);
    }

    fn delete_text(&mut self, mut k: i32) -> i32 {
        let mut deleted = 0;
        while k > 0 {
            let i = self.prev[self.cursor].unwrap();
            if self.chars[i] == b' ' {
                break;
            }
            deleted += 1;
            k -= 1;
            let j = self.prev[i].unwrap();
            self.next[j] = Some(self.cursor);
            self.prev[self.cursor] = Some(j);

            self.next[i] = None;
            self.prev[i] = None;
        }
        deleted
    }

    fn cursor_left(&mut self, mut k: i32) -> String {
        while k > 0 {
            let i = self.prev[self.cursor].unwrap();
            if self.chars[i] == b' ' {
                break;
            }
            k -= 1;
            self.cursor = i;
        }
        self.text_to_the_left(10)
    }

    fn cursor_right(&mut self, mut k: i32) -> String {
        while k > 0 {
            let i = self.next[self.cursor];
            if i.is_none() {
                assert!(self.chars[self.cursor] == b' ');
                break;
            }
            let i = i.unwrap();
            k -= 1;
            self.cursor = i;
        }
        self.text_to_the_left(10)
    }

    fn text_to_the_left(&self, mut max_len: usize) -> String {
        let mut text = Vec::new();
        let mut i = self.cursor;
        while max_len > 0 {
            i = self.prev[i].unwrap();
            if self.chars[i] == b' ' {
                break;
            }
            text.push(self.chars[i]);
            max_len -= 1;
        }
        text.reverse();
        from_u8(&text)
    }
}

/**
 * Your TextEditor object will be instantiated and called as such:
 * let obj = TextEditor::new();
 * obj.add_text(text);
 * let ret_2: i32 = obj.delete_text(k);
 * let ret_3: String = obj.cursor_left(k);
 * let ret_4: String = obj.cursor_right(k);
 */

fn to_u8(s: &String) -> Vec<u8> {
    s.as_bytes().iter().map(|&c| c as u8).collect()
}

fn to_u8_vec(s: &Vec<String>) -> Vec<Vec<u8>> {
    s.iter().map(|ss| to_u8(&ss)).collect()
}

fn from_u8(s: &Vec<u8>) -> String {
    String::from_utf8(s.to_vec()).unwrap()
}

fn from_u8_vec(s: &Vec<Vec<u8>>) -> Vec<String> {
    s.iter().map(|ss| from_u8(&ss)).collect()
}

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
