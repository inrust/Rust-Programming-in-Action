use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Student {
    name: &'static str,
    score: i32,
}

impl Student {
    pub fn new(name: &'static str, score: i32) -> Self {
        Student { name, score }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn score(&self) -> i32 {
        self.score
    }
}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Student) -> Option<Ordering> {
        return if self.score == other.score {
            Some(self.name().cmp(&other.name()))
        } else {
            Some(self.score().cmp(&other.score()))
        };
    }
}
