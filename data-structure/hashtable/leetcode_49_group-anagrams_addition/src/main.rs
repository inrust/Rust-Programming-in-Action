use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut vecs: Vec<Vec<String>> = Vec::new();
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for i in 0..strs.len() {
            let mut count = [0; 26];
            for c in strs[i].chars() {
                let index = (c as u32 - 'a' as u32) as usize;
                count[index] += 1;
            }

            let mut chars = vec![];
            for i in 0..count.len() {
                chars.push(count[i].to_string() + "#");
            }

            let key: String = chars.into_iter().collect();
            let value = map.get(&key);
            if value != None {
                let mut v = value.unwrap().to_vec();
                v.push(strs[i].clone());
                map.insert(key, v);
            } else {
                let v = vec![strs[i].clone()];
                map.insert(key, v);
            }
        }

        for val in map.values() {
//            vecs.push((*val).clone());
            vecs.push(val.to_vec());
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
