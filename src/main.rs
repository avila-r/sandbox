fn main() {
    println! {
        "Reverse Words in a String III: {}",
        reverse(String::from("Let's take LeetCode contest"))
    }
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
