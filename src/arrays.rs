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
            expected: Option<i32>,
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
}
