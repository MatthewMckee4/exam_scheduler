use crate::course::Course;

#[derive(Debug, Clone)]
pub(crate) struct Student {
    courses: Vec<Course>,
}

impl Student {
    pub fn new() -> Self {
        Self {
            courses: Vec::new(),
        }
    }

    pub fn courses(&self) -> &Vec<Course> {
        &self.courses
    }
}
