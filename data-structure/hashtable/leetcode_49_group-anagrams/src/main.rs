use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut vecs: Vec<Vec<String>> = Vec::new();
        let mut used: Vec<bool> = vec![false; strs.len()];

        for i in 0..strs.len() {
            let mut temp: Vec<String> = Vec::new();
            if !used[i] {
                temp.push(strs[i].clone());
                for j in i + 1..strs.len() {
                    let mut is_anagram: bool = true;
                    if strs[i].len() != strs[j].len() {
                        continue;
                    }

                    let mut map = HashMap::new();
                    for c in strs[i].chars() {
                        let count = map.entry(c).or_insert(0);
                        *count += 1;
                    }
                    for c in strs[j].chars() {
                        let count = map.entry(c).or_insert(0);
                        *count -= 1;
                        if *count < 0 {
                            is_anagram = false;
                            break;
                        }
                    }

                    if is_anagram {
                        used[j] = true;
                        temp.push(strs[j].clone());
                    }
                }
            }

            if !temp.is_empty() {
                vecs.push(temp);
            }
        }

        return vecs;
    }
}

fn main() {
    let eat = String::from("eat");
    let tea = String::from("tea");
    let tan = String::from("tan");
    let ate = String::from("ate");
    let nat = String::from("nat");
    let bat = String::from("bat");
    let strs: Vec<String> = vec![eat, tea, tan, ate, nat, bat];

    println!("{:?}", Solution::group_anagrams(strs));
}
