pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();

    for (i, &n) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - n)) {
            return vec![j as i32, i as i32];
        }
        map.insert(n, i);
    }

    vec![]
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
}
