use chrono::NaiveDate;
use std::fmt::Display;

#[derive(PartialEq, Eq)]
pub struct Task {
    pub title: String,
    pub date: NaiveDate,
    pub completed: bool,
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

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { tasks: vec![] }
    }
    pub fn new_task(&mut self, title: String, date: NaiveDate) {
        let task = Task::new(title, date);
        self.tasks.push(task);
    }
    pub fn get_tasks(&self) -> &[Task] {
        self.tasks.as_slice()
    }
    pub fn get_tasks_at_date(&self, date: NaiveDate) -> impl Iterator<Item = &'_ Task> {
        self.tasks.iter().filter(move |task| task.date == date)
    }
    pub fn get_tasks_at_date_mut(&mut self, date: NaiveDate) -> impl Iterator<Item = &'_ mut Task> {
        self.tasks.iter_mut().filter(move |task| task.date == date)
    }
    pub fn complete_tasks(tasks_to_complete: Vec<&mut Task>) {
        for task in tasks_to_complete {
            task.completed = true;
        }
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}
