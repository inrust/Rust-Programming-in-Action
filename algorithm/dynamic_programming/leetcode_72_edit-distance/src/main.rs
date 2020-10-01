struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1_chars: Vec<char> = word1.chars().collect();
        let word2_chars: Vec<char> = word2.chars().collect();
        let n1 = word1.len();
        let n2 = word2.len();
        let mut dp = vec![vec![0; n2 + 1]; n1 + 1];

        for j in 1..=n2 {
            dp[0][j] = dp[0][j - 1] + 1;
        }

        for i in 1..=n1 {
            dp[i][0] = dp[i - 1][0] + 1;
        }

        for i in 1..=n1 {
            for j in 1..=n2 {
                if word1_chars[i - 1] == word2_chars[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = dp[i][j - 1].min(dp[i - 1][j]).min(dp[i - 1][j - 1]) + 1;
                }
            }
        }

        dp[n1][n2]
    }
}

fn main() {
    let word1 = String::from("horse");
    let word2 = String::from("ros");

    println!("{}", Solution::min_distance(word1, word2));
}
