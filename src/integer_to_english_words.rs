//! Solution for https://leetcode.com/problems/integer-to-english-words
//! 273. Integer to English Words

use core::panic;

fn to_string(num: u32) -> String {
    if num < 10 {
        return match num {
            0 => "Zero".to_string(),
            1 => "One".to_string(),
            2 => "Two".to_string(),
            3 => "Three".to_string(),
            4 => "Four".to_string(),
            5 => "Five".to_string(),
            6 => "Six".to_string(),
            7 => "Seven".to_string(),
            8 => "Eight".to_string(),
            9 => "Nine".to_string(),
            _ => panic!("{} is not a digit", num),
        };
    }

    if num < 20 {
        return match num {
            10 => "Ten".to_string(),
            11 => "Eleven".to_string(),
            12 => "Twelve".to_string(),
            13 => "Thirteen".to_string(),
            14 => "Fourteen".to_string(),
            15 => "Fifteen".to_string(),
            16 => "Sixteen".to_string(),
            17 => "Seventeen".to_string(),
            18 => "Eighteen".to_string(),
            19 => "Nineteen".to_string(),
            _ => panic!("{}", num),
        };
    }

    if num < 100 {
        let tens = num / 10;
        let mut pref = match tens {
            2 => "Twenty".to_string(),
            3 => "Thirty".to_string(),
            4 => "Forty".to_string(),
            5 => "Fifty".to_string(),
            6 => "Sixty".to_string(),
            7 => "Seventy".to_string(),
            8 => "Eighty".to_string(),
            9 => "Ninety".to_string(),
            _ => panic!("{}", num),
        };
        let num_suff = num % 10;
        if num_suff > 0 {
            pref.push_str(" ");
            pref.push_str(&to_string(num_suff));
        }
        return pref;
    }

    if num < 1000 {
        return to_string_helper(num, 100, "Hundred");
    }

    if num < 1_000_000 {
        return to_string_helper(num, 1_000, "Thousand");
    }

    if num < 1_000_000_000 {
        return to_string_helper(num, 1_000_000, "Million");
    }

    return to_string_helper(num, 1_000_000_000, "Billion");
}

fn to_string_helper(num: u32, coef: u32, pref_name: &str) -> String {
    let num_pref = num / coef;
    assert!(num_pref >= 1);
    let mut pref = to_string(num_pref);
    pref.push_str(" ");
    pref.push_str(&pref_name);

    let num_suff = num % coef;
    if num_suff > 0 {
        let suff = to_string(num_suff);
        pref.push_str(" ");
        pref.push_str(&suff);
    }
    return pref;
}

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        return to_string(num as u32);
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(123, "One Hundred Twenty Three")]
    #[case(12345, "Twelve Thousand Three Hundred Forty Five")]
    #[case(
        1234567,
        "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
    )]
    fn case(#[case] num: i32, #[case] expected: String) {
        let actual = Solution::number_to_words(num);
        assert_eq!(actual, expected);
    }
}
