pub fn reverse_words_iii(s: String) -> String {
    let (mut chars, mut l) = (s.chars().collect::<Vec<char>>(), 0);

    for i in 0..=chars.len() {
        if i == chars.len() || chars[i] == ' ' {
            let mut r = i - 1;

            while l < r {
                chars.swap(l, r);
                l += 1;
                r -= 1;
            }

            l = i + 1;
        }
    }

    chars.iter().collect()
}

pub fn binary_search(numbers: Vec<i32>, target: i32, p: Option<(usize, usize)>) -> Option<usize> {
    let (mut left, mut right) = p.unwrap_or((0, numbers.len()));

    while left < right {
        let mid: usize = left + (right - left) / 2;

        match numbers[mid] {
            v if v.eq(&target) => return Some(mid),
            v if v < target => left = mid + 1,
            _ => right = mid,
        }
    }

    None
}

pub fn exponential_search(numbers: Vec<i32>, target: i32) -> Option<usize> {
    if numbers.first().eq(&Some(&target)) {
        return Some(0);
    }

    let right = (1..)
        .take_while(|&i| i < numbers.len() && numbers[i] < target)
        .last()
        .map_or(1, |i| i * 2)
        .min(numbers.len());

    if right < numbers.len() && numbers[right] == target {
        return Some(right);
    }

    match binary_search(numbers, target, Some((right / 2, right))) {
        Some(i) => Some(i),
        _ => None,
    }
}

pub fn max_substring_with_occurrences(s: String, max_occurrences: i32) -> Option<usize> {
    let mut left = 0;
    let (chars, mut frequencies, mut max) = (
        s.chars().collect::<Vec<char>>(),
        std::collections::HashMap::new(),
        0,
    );

    for right in 0..chars.len() {
        frequencies
            .entry(chars[right])
            .and_modify(|v| *v += 1)
            .or_insert(1);

        while frequencies.get(&chars[right]).unwrap_or(&0) > &max_occurrences {
            frequencies.entry(chars[left]).and_modify(|v| *v -= 1);
            left += 1;
        }

        max = max.max(right - left + 1);
    }

    Some(max)
}

pub fn first_unique_char(s: String) -> i32 {
    use std::collections::HashMap;

    let mut frequencies: HashMap<char, i32> = HashMap::new();

    s.chars().for_each(|c| {
        frequencies.entry(c).and_modify(|i| *i += 1).or_insert(1);
    });

    for (i, c) in s.chars().enumerate() {
        if frequencies[&c].eq(&1) {
            return i as i32;
        }
    }

    -1
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    use std::cmp::Ordering::{Equal, Greater, Less};

    let (mut left, mut right) = (0, nums.len() - 1);

    while left <= right {
        let mid = left + (right - left) / 2;

        match nums[mid].cmp(&target) {
            Less => left = mid + 1,
            Equal => return mid as i32,
            Greater => right = mid - 1,
        }
    }

    -1
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut appearances: HashMap<i32, usize> = HashMap::new();

    for (i, &n) in nums.iter().enumerate() {
        if let Some(&j) = appearances.get(&(target - n)) {
            return vec![j as i32, i as i32];
        }

        appearances.insert(n, i);
    }

    vec![]
}

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashMap;

    let mut appearances: HashMap<i32, usize> = HashMap::new();

    for (i, &n) in nums.iter().enumerate() {
        match appearances.get(&n) {
            Some(&j) if j.abs_diff(i) <= (k as usize) => return true,
            _ => (),
        };

        appearances.insert(n, i);
    }

    false
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut left = 0;
    for right in 1..nums.len() {
        if nums[left] != nums[right] {
            left += 1;
            nums[left] = nums[right];
        }
    }

    (left + 1) as i32
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut left = 0;
    for right in 0..nums.len() {
        if nums[right] != val {
            nums[left] = nums[right];
            left += 1;
        }
    }
    left as i32
}

pub fn search_insert_position(nums: Vec<i32>, target: i32) -> i32 {
    use std::cmp::Ordering::{Equal, Greater, Less};

    let (mut left, mut right) = (0, nums.len());

    while left < right {
        let mid = left + (right - left) / 2;

        match nums[mid].cmp(&target) {
            Less => left = mid + 1,
            Equal => return mid as i32,
            Greater => right = mid,
        };
    }

    left as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words_iii() {
        struct TestCase {
            input: String,
            expected: String,
        }

        let cases = vec![
            TestCase {
                input: String::from("Let's take LeetCode contest"),
                expected: String::from("s'teL ekat edoCteeL tsetnoc"),
            },
            TestCase {
                input: String::from("Mr Ding"),
                expected: String::from("rM gniD"),
            },
        ];

        cases.iter().for_each(|case| {
            let result = reverse_words_iii(case.input.clone());

            assert_eq! {result, case.expected};
        });
    }

    #[test]
    fn test_search_insert_position() {
        struct TestCase {
            numbers: Vec<i32>,
            target: i32,
            expected: i32,
        }

        let cases = vec![
            TestCase {
                numbers: vec![1, 3, 5, 6],
                target: 5,
                expected: 2,
            },
            TestCase {
                numbers: vec![1, 3, 5, 6],
                target: 2,
                expected: 1,
            },
            TestCase {
                numbers: vec![1, 3, 5, 6],
                target: 7,
                expected: 4,
            },
        ];

        cases.iter().for_each(|case| {
            let result = search_insert_position(case.numbers.clone(), case.target.clone());

            assert_eq! {result, case.expected};
        });
    }

    #[test]
    fn test_binary_search() {
        struct TestCase {
            numbers: Vec<i32>,
            target: i32,
            expected: Option<usize>,
        }

        let cases = vec![
            TestCase {
                numbers: vec![1, 3, 5, 6],
                target: 5,
                expected: Some(2),
            },
            TestCase {
                numbers: vec![1, 3, 5, 6],
                target: 2,
                expected: None,
            },
            TestCase {
                numbers: vec![1, 3, 5, 6],
                target: 7,
                expected: None,
            },
            TestCase {
                numbers: vec![1, 3, 5, 6],
                target: 1,
                expected: Some(0),
            },
            TestCase {
                numbers: vec![1, 3, 5, 6],
                target: 6,
                expected: Some(3),
            },
        ];

        cases.iter().for_each(|case| {
            let result = binary_search(case.numbers.clone(), case.target, None);

            assert_eq! {result, case.expected};
        });
    }

    #[test]
    fn test_exponential_search() {
        struct TestCase {
            numbers: Vec<i32>,
            target: i32,
            expected: Option<usize>,
        }

        let cases = vec![
            TestCase {
                numbers: vec![1, 3, 5, 6],
                target: 5,
                expected: Some(2),
            },
            TestCase {
                numbers: vec![1, 3, 5, 6],
                target: 2,
                expected: None,
            },
            TestCase {
                numbers: vec![1, 3, 5, 6],
                target: 7,
                expected: None,
            },
            TestCase {
                numbers: vec![1, 3, 5, 6],
                target: 1,
                expected: Some(0),
            },
            TestCase {
                numbers: vec![1, 3, 5, 6],
                target: 6,
                expected: Some(3),
            },
            TestCase {
                numbers: vec![],
                target: 5,
                expected: None,
            },
            TestCase {
                numbers: vec![3],
                target: 3,
                expected: Some(0),
            },
        ];

        cases.iter().for_each(|case| {
            let result = exponential_search(case.numbers.clone(), case.target);

            assert_eq!(result, case.expected);
        });
    }

    #[test]
    fn test_maximum_length_substring_with_occurrences() {
        struct TestCase {
            s: String,
            max_occurrences: i32,
            expected: Option<usize>,
        }

        let cases = vec![
            TestCase {
                s: "bcbbbcba".to_string(),
                max_occurrences: 2,
                expected: Some(4),
            },
            TestCase {
                s: "abcabcbb".to_string(),
                max_occurrences: 1,
                expected: Some(3),
            },
            TestCase {
                s: "aaaaaa".to_string(),
                max_occurrences: 2,
                expected: Some(2),
            },
            TestCase {
                s: "abccba".to_string(),
                max_occurrences: 2,
                expected: Some(6),
            },
            TestCase {
                s: "".to_string(),
                max_occurrences: 2,
                expected: Some(0),
            },
        ];

        cases.iter().for_each(|case| {
            let result = max_substring_with_occurrences(case.s.clone(), case.max_occurrences);

            assert_eq!(result, case.expected);
        });
    }

    #[test]
    fn test_first_unique_char() {
        struct TestCase {
            s: String,
            expected: i32,
        }

        let cases = vec![
            TestCase {
                s: "leetcode".to_string(),
                expected: 0,
            },
            TestCase {
                s: "loveleetcode".to_string(),
                expected: 2,
            },
            TestCase {
                s: "aabb".to_string(),
                expected: -1,
            },
            TestCase {
                s: "abcd".to_string(),
                expected: 0,
            },
            TestCase {
                s: "".to_string(),
                expected: -1,
            },
        ];

        cases.iter().for_each(|case| {
            let result = first_unique_char(case.s.clone());

            assert_eq!(result, case.expected);
        });
    }

    #[test]
    fn test_search() {
        struct TestCase {
            nums: Vec<i32>,
            target: i32,
            expected: i32,
        }

        let cases = vec![
            TestCase {
                nums: vec![-1, 0, 3, 5, 9, 12],
                target: 9,
                expected: 4,
            },
            TestCase {
                nums: vec![-1, 0, 3, 5, 9, 12],
                target: 2,
                expected: -1,
            },
            TestCase {
                nums: vec![1, 2, 3, 4, 5],
                target: 3,
                expected: 2,
            },
            TestCase {
                nums: vec![10, 20, 30, 40, 50],
                target: 40,
                expected: 3,
            },
            TestCase {
                nums: vec![100, 200, 300, 400, 500],
                target: 600,
                expected: -1,
            },
            TestCase {
                nums: vec![0, 1, 2, 3, 4],
                target: 5,
                expected: -1,
            },
            TestCase {
                nums: vec![2, 4, 6, 8, 10],
                target: 4,
                expected: 1,
            },
        ];

        cases.iter().for_each(|case| {
            let result = search(case.nums.clone(), case.target);

            assert_eq!(result, case.expected);
        });
    }

    #[test]
    fn test_two_sum() {
        struct TestCase {
            numbers: Vec<i32>,
            target: i32,
            expected: Vec<i32>,
        }

        let test_cases = vec![
            TestCase {
                numbers: vec![2, 7, 11, 15],
                target: 9,
                expected: vec![0, 1],
            },
            TestCase {
                numbers: vec![3, 2, 4],
                target: 6,
                expected: vec![1, 2],
            },
            TestCase {
                numbers: vec![3, 3],
                target: 6,
                expected: vec![0, 1],
            },
            TestCase {
                numbers: vec![1, 2, 3],
                target: 7,
                expected: vec![],
            },
        ];

        test_cases.iter().for_each(|case| {
            assert_eq!(
                two_sum(case.numbers.clone(), case.target.clone()),
                case.expected
            );
        });
    }

    #[test]
    fn test_nearby_duplicate() {
        struct TestCase {
            numbers: Vec<i32>,
            max_distance: i32,
            expected: bool,
        }

        let cases = vec![
            TestCase {
                numbers: vec![1, 2, 3, 1],
                max_distance: 3,
                expected: true,
            },
            TestCase {
                numbers: vec![1, 0, 1, 1],
                max_distance: 1,
                expected: true,
            },
            TestCase {
                numbers: vec![1, 2, 3, 1, 2, 3],
                max_distance: 2,
                expected: false,
            },
        ];

        for case in cases {
            assert_eq! {
               contains_nearby_duplicate(case.numbers.clone(), case.max_distance.clone()),
               case.expected
            };
        }
    }

    #[test]
    fn test_remove_duplicates() {
        let (mut input, expected) = (vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], 5);

        let result = remove_duplicates(&mut input);

        assert_eq! { result, expected };
    }

    #[test]
    fn test_remove_element() {
        let ((mut numbers, val), expected) = ((vec![0, 1, 2, 2, 3, 0, 4, 2], 2), 5);

        assert_eq! { remove_element(&mut numbers, val), expected }
    }
}
