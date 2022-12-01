use std::fs;

#[derive(Clone)]
struct Entry {
    day: fn(String),
    data_path: String,
    example_data_path: String,
}

pub struct Runner {
    days: Vec<Entry>,
}

impl Runner {
    pub fn new() -> Self {
        Runner { days: Vec::new() }
    }

    pub fn add_day(&mut self, day: fn(String), data_path: String, example_data_path: String) {
        self.days.push(Entry {
            day,
            data_path,
            example_data_path,
        });
    }

    pub fn run(&self) {
        for (i, day) in self.days.clone().into_iter().enumerate() {
            let cont = fs::read_to_string(day.data_path).unwrap();
            println!("---- Day {} ----", i + 1);
            (day.day)(cont);
        }
    }

    pub fn run_example(&self) {
        for (i, day) in self.days.clone().into_iter().enumerate() {
            let cont = fs::read_to_string(day.example_data_path).unwrap();
            println!("---- Day {} ----", i + 1);
            (day.day)(cont);
        }
    }
}
