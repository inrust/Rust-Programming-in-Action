struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut merged: Vec<Vec<i32>> = Vec::new();
        if intervals.len() == 0 { return merged; }

        intervals.sort();

        for i in 0..intervals.len() {
            let len = merged.len();
            if merged.is_empty() || merged[len-1][1] < intervals[i][0] {
                merged.push(intervals[i].clone());
            } else {
                merged[len-1][1] = merged[len-1][1].max(intervals[i][1]);
            }
        }

        return merged;
    }
}

fn main() {
    let mut intervals = Vec::new();
    intervals.push(vec![15, 18]);
    intervals.push(vec![2, 6]);
    intervals.push(vec![8, 10]);
    intervals.push(vec![1, 3]);

    println!("{:?}", Solution::merge(intervals));
}
