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

pub fn bubble_sorted(nums: Vec<i32>) -> Vec<i32> {
    let mut new = nums.clone();
    self::bubble_sort(&mut new);
    new
}

pub fn quicksort(nums: &mut Vec<i32>, pointers: Option<(usize, usize)>) {
    if nums.is_empty() {
        return;
    }

    let (left, right) = pointers.unwrap_or((0, nums.len() - 1));

    if left < right {
        let partition = (|| {
            let pivot = nums[right];

            let mut i = None;
            for j in left..right {
                if nums[j] <= pivot {
                    i = Some(i.map_or(left, |v| v + 1));
                    nums.swap(i.unwrap(), j);
                }
            }

            let i = i.map_or(left, |v| v + 1);
            nums.swap(i, right);
            i
        })();

        quicksort(nums, Some((left, partition.saturating_sub(1))));
        quicksort(nums, Some((partition + 1, right)));
    }
}

pub fn quicksorted(nums: Vec<i32>) -> Vec<i32> {
    let mut sorted = nums.clone();
    self::quicksort(&mut sorted, None);
    sorted
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

            let result = bubble_sorted(case.input.clone());

            assert_eq! { nums, case.expected };
            assert_eq! { result, case.expected };
        });
    }

    #[test]
    fn test_quicksort() {
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
            quicksort(&mut nums, None);

            let result = quicksorted(case.input.clone());

            assert_eq! { nums, case.expected };
            assert_eq! { result, case.expected };
        });
    }
}
