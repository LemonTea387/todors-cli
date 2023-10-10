use chrono::NaiveDate;
use inquire::{error::InquireResult, DateSelect, Select, Text};
use std::fmt::Display;

static PROMPT: &str = "TODO-RS>";

fn main() -> InquireResult<()> {
    loop {
        let command = Select::new(PROMPT, get_categories()).prompt().unwrap_or("");
        match command {
            "Add Task" => add_task()?,
            _ => break,
        }
    }
    Ok(())
}

fn add_task() -> InquireResult<()> {
    let date = DateSelect::new("Date:").prompt()?;
    let title = Text::new("Task: ").prompt().unwrap_or("".to_owned());

    let task = Task::new(title, date);
    println!("Created Task {}", task);

    Ok(())
}

struct Task {
    title: String,
    date: NaiveDate,
}

impl Task {
    fn new(title: String, date: NaiveDate) -> Self {
        Task { title, date }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.title, self.date)
    }
}

fn get_categories() -> Vec<&'static str> {
    vec!["Add Task", "Quit"]
}
