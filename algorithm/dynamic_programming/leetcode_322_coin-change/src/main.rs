struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; (amount + 1) as usize];
        dp[0] = 0;
        for i in 1..=amount {
            for &coin in coins.iter() {
                if i >= coin {
                    let i = i as usize;
                    let coin = coin as usize;
                    dp[i] = dp[i].min(dp[i - coin] + 1);
                }
            }
        }

        let last = *dp.last().unwrap();
        if last > amount { -1 } else { last }
    }
}

fn main() {
    let coins = vec![1, 3, 5];
    println!("{}", Solution::coin_change(coins, 11));
}
