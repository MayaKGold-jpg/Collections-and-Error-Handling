use std::collections::HashMap;

/// Struct that wraps the student database
pub struct StudentDatabase {
    pub data: HashMap<String, Vec<f32>>,
}

impl StudentDatabase {
    pub fn new() -> Self {
        StudentDatabase {
            data: HashMap::new(),
        }
    }

    pub fn add_grade(&mut self, student: &str, grade: f32) {
        self.data
            .entry(student.to_string())
            .or_insert(Vec::new())
            .push(grade);
    }

    pub fn get_average(&self, student: &str) -> Result<f32, String> {
        match self.data.get(student) {
            Some(grades) => {
                let sum: f32 = grades.iter().sum();
                Ok(sum / grades.len() as f32)
            }
            None => Err(format!("Student '{}' does not exist", student)),
        }
    }

    pub fn list_all_averages(&self) -> Vec<(String, f32)> {
        self.data
            .iter()
            .map(|(name, grades)| {
                let avg = grades.iter().sum::<f32>() / grades.len() as f32;
                (name.clone(), avg)
            })
            .collect()
    }
}

fn parse_grade(input: &str) -> Result<f32, String> {
    input
        .trim()
        .parse::<f32>()
        .map_err(|_| format!("Invalid grade '{}'", input))
}

fn main() {
    let mut db = StudentDatabase::new();

    let inputs = vec![
        ("Alice", "90.5"),
        ("Alice", "85.0"),
        ("Bob", "78.0"),
        ("Charlie", "A+"),
    ];

    for (name, grade_str) in inputs {
        match parse_grade(grade_str) {
            Ok(grade) => db.add_grade(name, grade),
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("\nAlice average:");
    match db.get_average("Alice") {
        Ok(avg) => println!("{:.2}", avg),
        Err(e) => println!("{}", e),
    }

    println!("\nAll students:");
    for (name, avg) in db.list_all_averages() {
        println!("{} -> {:.2}", name, avg);
    }
}