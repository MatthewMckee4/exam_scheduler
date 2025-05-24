use std::time::Duration;

use crate::student::Student;

#[derive(Debug, Clone)]
pub(crate) struct Course {
    name: String,
    enrolled_students: Vec<Student>,
    exam: Exam,
}

impl Course {
    pub fn new(name: String, enrolled_students: Vec<Student>, exam: Exam) -> Self {
        Self {
            name,
            enrolled_students,
            exam,
        }
    }

    pub fn students(&self) -> &Vec<Student> {
        &self.enrolled_students
    }

    pub fn exam(&self) -> &Exam {
        &self.exam
    }
}

#[derive(Debug, Clone)]
struct Exam {
    duration: Duration,
}

impl Exam {
    pub fn new(duration: impl Into<Duration>) -> Self {
        Self {
            duration: duration.into(),
        }
    }

    pub fn duration(&self) -> &Duration {
        &self.duration
    }
}
