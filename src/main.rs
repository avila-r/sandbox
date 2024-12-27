pub mod arrays;

fn main() {
    println! {
        "Reverse Words in a String III: {}",
        reverse(String::from("Let's take LeetCode contest"))
    }

    println! {
        "Binary search: {}",
        match binary_search(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 3, None) {
            Some(i) => format!("Target was found in index {i}"),
            None => "Target not found".to_string()
        }
    }

    println! {
        "Generic Sliding Window: {}",
        match sliding_window("bcbbbcba".chars().collect()) {
            Some(i) => format!("Maximum substring length with two occurrences: {i}"),
            _ => "?".to_string(),
        }
    }

    println! {
        "Maximum Length Substring With Two Occurrences: {}",
        match max_substring("bcbbbcba".to_string(), 2) {
            Some(i) => format!("Result: {i}"),
            _ => "Not found".to_string(),
        },
    }

    println! {
        "Leet Code's Binary Search: {}",
        (|numbers: Vec<i32>, target: i32| {
            use std::cmp::Ordering::{Less, Equal, Greater};

            let (mut left, mut right) = (0, numbers.len());
            for _ in 0..right {
                let mid = left + (right - left) / 2;

                match numbers[mid].cmp(&target) {
                    Less => left = mid + 1,
                    Equal => return mid as i32,
                    Greater => right = mid,
                }
            }

            -1
        })(vec![-1, 0, 3, 5, 9, 12], 9)
    }

    println! {
        "Exponential Search: {}",
        ((|numbers: Vec<i32>, target: i32| {
            if numbers.first().eq(&Some(&target)) {
                return 0;
            }

            let right = (1..)
                .take_while(|&i| i < numbers.len() && numbers[i] < target)
                .last()
                .map_or(1, |i| i * 2)
                .min(numbers.len());

            if right < numbers.len() && numbers[right] == target {
                return right as i32;
            }

            match binary_search(numbers, target, Some((right / 2, right))) {
                Some(i) => i as i32,
                None => -1,
            }
        })(vec![-1, 0, 3, 5, 9, 12], 9))
    }

    println! {
        "Leet Code's First Unique Character in a String: {}",
        first_unique_char("leetcode".to_string())
    }
}

fn first_unique_char(s: String) -> i32 {
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

fn max_substring(s: String, max_occurrences: i32) -> Option<usize> {
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

fn sliding_window(chars: Vec<char>) -> Option<i32> {
    let (mut left, mut right) = (0, 0);
    let (mut frequencies, mut max) = (std::collections::HashMap::new(), 0);

    while right < chars.len() {
        frequencies
            .entry(chars[right])
            .and_modify(|v| *v += 1)
            .or_insert(1);

        while frequencies.get(&chars[right]).unwrap_or(&0) > &2 {
            frequencies.entry(chars[left]).and_modify(|v| *v -= 1);
            left += 1;
        }

        max = max.max(right - left + 1);
        right += 1;
    }

    Some(max as i32)
}

fn binary_search(numbers: Vec<i32>, target: i32, ptrs: Option<(usize, usize)>) -> Option<usize> {
    let (mut left, mut right) = ptrs.unwrap_or((0, numbers.len()));

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

fn reverse(s: String) -> String {
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
