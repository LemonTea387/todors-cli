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
            "Complete Tasks At" => mark_tasks_at_date(&mut tm).ok(),
            _ => break,
        };
    }
    Ok(())
}

fn add_task(tm: &mut TaskManager) -> InquireResult<()> {
    let date = DateSelect::new("Date:").prompt()?;
    let title = Text::new("Task: ").prompt().unwrap_or("".to_owned());

    tm.new_task(title, date);

    Ok(())
}

fn list_tasks(tm: &TaskManager) -> InquireResult<()> {
    for task in tm.get_tasks() {
        println!("[{}] {}", if task.completed { "x" } else { " " }, task);
    }
    Ok(())
}

fn list_tasks_at_date(tm: &TaskManager) -> InquireResult<()> {
    let date = DateSelect::new("Date:").prompt()?;
    for task in tm.get_tasks_at_date(&date) {
        println!("[{}] {}", if task.completed { "x" } else { " " }, task);
    }

    Ok(())
}

fn mark_tasks_at_date(tm: &mut TaskManager) -> InquireResult<()> {
    let date = DateSelect::new("Date:").prompt()?;
    let tasks_to_complete = MultiSelect::new(
        "Mark Tasks as Completed",
        tm.get_tasks_at_date_mut(&date).collect(),
    )
    .prompt()?;

    TaskManager::complete_tasks(tasks_to_complete);

    Ok(())
}

#[derive(PartialEq, Eq)]
struct Task {
    id: u32,
    title: String,
    date: NaiveDate,
    completed: bool,
}

impl Task {
    fn new(id: u32, title: String, date: NaiveDate) -> Self {
        Task {
            id,
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

    fn new_id(&self) -> u32 {
        if !self.tasks.is_empty() {
            return self.tasks.last().unwrap().id + 1;
        }
        0
    }
    fn new_task(&mut self, title: String, date: NaiveDate) {
        let task = Task::new(self.new_id(), title, date);
        self.tasks.push(task);
    }
    fn get_tasks(&self) -> &[Task] {
        self.tasks.as_slice()
    }
    fn get_tasks_at_date<'a>(&'a self, date: &'a NaiveDate) -> impl Iterator<Item = &'a Task> {
        self.tasks.iter().filter(move |task| task.date == *date)
    }
    fn get_tasks_at_date_mut<'a>(
        &'a mut self,
        date: &'a NaiveDate,
    ) -> impl Iterator<Item = &'a mut Task> {
        self.tasks.iter_mut().filter(move |task| task.date == *date)
    }
    fn complete_tasks(tasks_to_complete: Vec<&mut Task>) {
        for task in tasks_to_complete {
            task.completed = true;
        }
    }
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
