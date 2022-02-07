/**
 *
 * Given students' names along with the grade that they are in, create a roster for the school.
In the end, you should be able to:

Add a student's name to the roster for a grade
"Add Jim to grade 2."
"OK."
Get a list of all students enrolled in a grade
"Which students are in grade 2?"
"We've only got Jim just now."
Get a sorted list of all students in all grades. Grades should sort as 1, 2, 3, etc., and students within a grade should be sorted alphabetically by name.
"Who all is enrolled in school right now?"
"Let me think. We have Anna, Barb, and Charlie in grade 1, Alex, Peter, and Zoe in grade 2 and Jim in grade 5. So the answer is: Anna, Barb, Charlie, Alex, Peter, Zoe and Jim"
Note that all our students only have one name. (It's a small town, what do you want?)
 */

pub mod grade_school {

  use std::collections::HashMap;

  pub struct School(HashMap<u32, Vec<String>>);

  impl School {
    pub fn new() -> School {
      School(HashMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
      self.0.entry(grade).or_insert(vec![]);
      self
        .0
        .entry(grade)
        .and_modify(|students| students.push(String::from(student)));
    }

    pub fn grades(&self) -> Vec<u32> {
      let mut grades = self.0.keys().cloned().collect::<Vec<u32>>();
      grades.sort();
      grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
      match self.0.get(&grade) {
        Some(students) => {
          let mut grade = students
            .iter()
            .map(|student| student.clone())
            .collect::<Vec<String>>();
          grade.sort();
          grade
        }
        None => Vec::new(),
      }
    }
  }
}

#[cfg(test)]
mod test {

  use super::grade_school as school;
  fn to_owned(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
  }
  #[test]
  fn test_grades_for_empty_school() {
    let s = school::School::new();
    let v: Vec<u32> = vec![];
    assert_eq!(s.grades(), v);
  }
  #[test]
  fn test_grades_for_one_student() {
    let mut s = school::School::new();
    s.add(2, "Aimee");
    assert_eq!(s.grades(), vec![2]);
  }
  #[test]
  fn test_grades_for_several_students_are_sorted() {
    let mut s = school::School::new();
    s.add(2, "Aimee");
    s.add(7, "Logan");
    s.add(4, "Blair");
    assert_eq!(s.grades(), vec![2, 4, 7]);
  }
  #[test]
  fn test_grades_when_several_students_have_the_same_grade() {
    let mut s = school::School::new();
    s.add(2, "Aimee");
    s.add(2, "Logan");
    s.add(2, "Blair");
    assert_eq!(s.grades(), vec![2]);
  }
  #[test]
  fn test_grade_for_empty_school() {
    let s = school::School::new();
    assert_eq!(s.grade(1), Vec::<String>::new());
  }
  #[test]
  fn test_grade_when_no_students_have_that_grade() {
    let mut s = school::School::new();
    s.add(7, "Logan");
    assert_eq!(s.grade(1), Vec::<String>::new());
  }
  #[test]
  fn test_grade_for_one_student() {
    let mut s = school::School::new();
    s.add(2, "Aimee");
    assert_eq!(s.grade(2), to_owned(&["Aimee"]));
  }
  #[test]
  fn test_grade_returns_students_sorted_by_name() {
    let mut s = school::School::new();
    s.add(2, "James");
    s.add(2, "Blair");
    s.add(2, "Paul");
    assert_eq!(s.grade(2), to_owned(&["Blair", "James", "Paul"]));
  }
  #[test]
  fn test_add_students_to_different_grades() {
    let mut s = school::School::new();
    s.add(3, "Chelsea");
    s.add(7, "Logan");
    assert_eq!(s.grades(), vec![3, 7]);
    assert_eq!(s.grade(3), to_owned(&["Chelsea"]));
    assert_eq!(s.grade(7), to_owned(&["Logan"]));
  }
}
