//! Solution for https://leetcode.com/problems/text-justification
//! 68. Text Justification

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let w = max_width as usize;
        let n = words.len();
        let mut ans = Vec::new();

        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            let mut cur_w = words[i].len();
            while j < n && cur_w + words[j].len() + 1 <= w {
                cur_w += words[j].len() + 1;
                j += 1;
            }

            let cnt = j - i;
            assert!(cur_w >= cnt - 1);
            let sum_len = cur_w - (cnt - 1);
            let mut row = String::new();
            for k in i..j {
                if k == i {
                    row += &words[k];
                    continue;
                }
                // last row
                if j == n {
                    row += " ";
                    row += &words[k];
                    continue;
                }
                // not last row
                assert!(cnt > 1);
                assert!(w >= sum_len);
                let mut spaces = (w - sum_len) / (cnt - 1);
                let rem = (w - sum_len) % (cnt - 1);
                if k - i <= rem {
                    spaces += 1;
                }
                row += &" ".repeat(spaces);
                row += &words[k];
            }
            assert!(row.len() <= w);
            row += &" ".repeat(w - row.len());

            ans.push(row);

            i = j;
        }

        ans
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    fn case() {
        let words = [
            "This",
            "is",
            "an",
            "example",
            "of",
            "text",
            "justification.",
        ];
        let max_width = 16;
        let expected = ["This    is    an", "example  of text", "justification.  "];

        let words: Vec<_> = words.iter().map(|s| s.to_string()).collect();
        let actual = Solution::full_justify(words, max_width);
        assert_eq!(actual, expected);
    }
}
