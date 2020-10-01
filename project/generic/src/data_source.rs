mod student;

use student::Student;

pub fn integer() -> Vec<i32> {
    return vec![1, 3, 6, 7, 8, 5, 4, 2, 10, 9];
}

pub fn floating_point() -> Vec<f64> {
    return vec![2.2, 1.1, 6.6, 5.5, 4.4, 3.3];
}

pub fn str() -> Vec<&'static str> {
    return vec!["B", "C", "D", "A", "G", "F", "E"];
}

pub fn student() -> Vec<Student> {
    return vec![Student::new("D", 90),
                Student::new("C", 100),
                Student::new("B", 95),
                Student::new("A", 95)];
}
