#[derive(Debug, PartialEq)]
pub struct Student {
    name: &'static str,
    score: i32,
}

impl Student {
    pub fn new(name: &'static str, score: i32) -> Self {
        Student { name, score }
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn set_score(&mut self, score: i32) {
        self.score = score;
    }
}

fn main() {
    let mut student: Student = Student::new("zhangsan", 59);
    println!("name: {}, score: {}", student.get_name(), student.get_score());

    student.set_score(60);
    println!("{:?}", student);
}
