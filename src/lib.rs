use chrono::NaiveDate;
use std::fmt::Display;

#[derive(PartialEq, Eq)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub date: NaiveDate,
    pub completed: bool,
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

pub struct TaskManager {
    tasks: Vec<Task>,
    highest_id: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks: vec![],
            highest_id: 0,
        }
    }
    pub fn new_task(&mut self, title: String, date: NaiveDate) {
        let task = Task::new(self.highest_id + 1, title, date);
        self.add_task(task);
    }
    fn add_task(&mut self, task: Task) {
        if task.id > self.highest_id {
            self.highest_id = task.id
        }
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
    pub fn delete_tasks(&mut self, tasks_id_to_delete: &[u32]) {
        for &id in tasks_id_to_delete {
            self.tasks.retain(|t| t.id != id)
        }
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}
