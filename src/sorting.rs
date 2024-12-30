pub fn bubble_sort(nums: &mut Vec<i32>) {
    let size = nums.len();
    if size <= 1 {
        return;
    }

    for _ in 0..size {
        let mut sorted = true;
        for i in 0..(size - 1) {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
                sorted = false;
            }
        }

        if sorted {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        struct TestCase {
            input: Vec<i32>,
            expected: Vec<i32>,
        }

        let cases = vec![
            TestCase {
                input: vec![5, 3, 8, 6, 2],
                expected: vec![2, 3, 5, 6, 8],
            },
            TestCase {
                input: vec![1, 2, 3, 4, 5],
                expected: vec![1, 2, 3, 4, 5],
            },
            TestCase {
                input: vec![9, 7, 5, 3, 1],
                expected: vec![1, 3, 5, 7, 9],
            },
            TestCase {
                input: vec![],
                expected: vec![],
            },
            TestCase {
                input: vec![42],
                expected: vec![42],
            },
            TestCase {
                input: vec![4, 1, 3, 2],
                expected: vec![1, 2, 3, 4],
            },
        ];

        cases.iter().for_each(|case| {
            let mut nums = case.input.clone();
            bubble_sort(&mut nums);
            assert_eq! { nums, case.expected };
        });
    }
}
