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

#[cfg(test)]
mod tests {
    use super::*;

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
}
