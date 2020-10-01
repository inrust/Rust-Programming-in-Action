use std::cmp::Ordering;

#[derive(Eq)]
struct Person {
    id: u32,
    name: String,
    height: u32,
}

impl Ord for Person {
    fn cmp(&self, other: &Person) -> Ordering {
        self.height.cmp(&other.height)
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Person) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Person) -> bool {
        self.height == other.height
    }
}

fn main() {
    let person1 = Person { id: 1, name: String::from("zhangsan"), height: 168 };
    let person2 = Person { id: 2, name: String::from("lisi"), height: 175 };
    let person3 = Person { id: 3, name: String::from("wanwu"), height: 180 };

    assert_eq!(person1 < person2, true);
    assert_eq!(person2 > person3, false);

    assert!(person1.lt(&person2));
    assert!(person3.gt(&person2));

    let tallest_person = person1.max(person2).max(person3);
    println!("id: {}, name: {}", tallest_person.id, tallest_person.name);
}
