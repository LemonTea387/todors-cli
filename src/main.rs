use chrono::NaiveDate;
use inquire::{error::InquireResult, DateSelect, MultiSelect, Select, Text};
use std::fmt::Display;

static PROMPT: &str = "TODO-RS>";

fn main() -> InquireResult<()> {
    let mut tm = TaskManager::new();
    loop {
        let command = Select::new(PROMPT, get_categories()).prompt().unwrap_or("");
        match command {
            "Add Task" => add_task(&mut tm).ok(),
            "List Tasks" => list_tasks(&tm).ok(),
            "List Tasks At" => list_tasks_at_date(&tm).ok(),
            "Complete Tasks At" => mark_tasks_at_date(&tm).ok(),
            _ => break,
        };
    }
    Ok(())
}

fn add_task(tm: &mut TaskManager) -> InquireResult<()> {
    let date = DateSelect::new("Date:").prompt()?;
    let title = Text::new("Task: ").prompt().unwrap_or("".to_owned());

    let task = Task::new(title, date);

    tm.new_task(task);

    Ok(())
}

fn list_tasks(tm: &TaskManager) -> InquireResult<()> {
    for task in tm.get_tasks() {
        println!("{}", task);
    }
    Ok(())
}

fn list_tasks_at_date(tm: &TaskManager) -> InquireResult<()> {
    let date = DateSelect::new("Date:").prompt()?;
    for task in tm.get_tasks_at_date(&date) {
        println!("{}", task);
    }

    Ok(())
}

fn mark_tasks_at_date(tm: &TaskManager) -> InquireResult<()> {
    let date = DateSelect::new("Date:").prompt()?;
    MultiSelect::new(
        "Mark Tasks as Completed",
        tm.get_tasks_at_date(&date).collect(),
    )
    .prompt()?;

    Ok(())
}

struct Task {
    title: String,
    date: NaiveDate,
    completed: bool,
}

impl Task {
    fn new(title: String, date: NaiveDate) -> Self {
        Task {
            title,
            date,
            completed: false,
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.title, self.date)
    }
}

struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager { tasks: vec![] }
    }
    fn new_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
    fn get_tasks(&self) -> &[Task] {
        self.tasks.as_slice()
    }
    fn get_tasks_at_date<'a>(&'a self, date: &'a NaiveDate) -> impl Iterator<Item = &'a Task> {
        self.tasks.iter().filter(move |&task| task.date == *date)
    }
    fn complete_tasks(&mut self, task: &[&Task]) {}
}

fn get_categories() -> Vec<&'static str> {
    vec![
        "Add Task",
        "List Tasks",
        "List Tasks At",
        "Complete Tasks At",
        "Quit",
    ]
}
