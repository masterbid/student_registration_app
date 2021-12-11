use std::io;

struct Student {
    name: String,
    age: i32,
    score: i32,
}

struct Class {
    name: String,
    students: Vec<Student>,
}

impl Class {
    fn calculate_avg_score(&self) -> i32 {
        let mut avg = 0;

        for i in self.students.iter() {
            avg += i.score;
        }
        avg /= self.students.len() as i32;
        avg
    }
    fn add_student(&mut self, student:Student) {
        self.students.push(student);
    }
    fn create_class(name: String, students:Vec<Student>) -> Class {
        Class{name, students}
    }
}

fn main() {
    let vec_students = vec![];
    let mut max_entry = 10;
    let mut myclass = Class::create_class("Computer Science".to_string(), vec_students);
    while max_entry > 0 {
        let mut buffer = String::from("");
        println!("Please enter student name, age and score, separated by comma: ");
        io::stdin().read_line(&mut buffer).expect("Error Occurred");
        let answer: Vec<&str> = buffer.split(",").collect();
        assert_eq!(answer.len(), 3);
        let name = answer[0].trim().to_string();
        let age: i32 = answer[1].trim().parse().unwrap();
        let score: i32 = answer[2].trim().parse().unwrap();
        let student = Student { name, age, score };
        println!(
            "You entered student name {} and age {} and score {}",
            student.name, student.age, student.score
        );
        myclass.add_student(student);
        max_entry -= 1;
    }
    
    println!(
        "Average score for class {} is {} out of 10 ",
        myclass.name,
        myclass.calculate_avg_score()
    );
}
