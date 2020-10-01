struct Solution;

impl Solution {
    pub fn min_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        if m == 0 { return 0; }
        let n = matrix[0].len();
        if n == 0 { return 0; }

        let mut states = vec![vec![0; n]; m];
        let mut sum = 0;
        for j in 0..n {
            sum += matrix[0][j];
            states[0][j] = sum;
        }

        sum = 0;
        for i in 0..m {
            sum += matrix[i][0];
            states[i][0] = sum;
        }

        for i in 1..m {
            for j in 1..n {
                states[i][j] = matrix[i][j] + states[i - 1][j].min(states[i][j - 1]);
            }
        }

        states[m - 1][n - 1]
    }
}

fn main() {
    let matrix = vec![
        vec![1, 3, 1],
        vec![1, 5, 1],
        vec![4, 2, 1],
    ];

    println!("{}", Solution::min_path_sum(matrix));
}
