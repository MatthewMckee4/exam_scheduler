use crate::course::Course;

#[derive(Debug, Clone)]
struct Scheduler {
    courses: Vec<Course>,
}

impl Scheduler {
    pub fn new(courses: Vec<Course>) -> Self {
        Self { courses }
    }
}
