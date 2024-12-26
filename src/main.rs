fn main() {
    println! {
        "Reverse Words in a String III: {}",
        reverse(String::from("Let's take LeetCode contest"))
    }

    println! {
        "Binary search: {}",
        match binary_search(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 3) {
            Some(i) => format!("Target was found in index {i}"),
            None => "Target not found".to_string()
        }
    }
}

fn binary_search(numbers: Vec<i32>, target: i32) -> Option<usize> {
    let (mut left, mut right) = (0, numbers.len());

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
