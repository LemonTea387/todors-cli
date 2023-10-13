use inquire::{error::InquireResult, DateSelect, MultiSelect, Select, Text};

use todors_cli::TaskManager;

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
            "Delete Tasks At" => delete_tasks_at_date(&mut tm).ok(),
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
    for task in tm.get_tasks_at_date(date) {
        println!("[{}] {}", if task.completed { "x" } else { " " }, task);
    }

    Ok(())
}

fn mark_tasks_at_date(tm: &mut TaskManager) -> InquireResult<()> {
    let date = DateSelect::new("Date:").prompt()?;
    let tasks_to_complete = MultiSelect::new(
        "Mark Tasks as Completed",
        tm.get_tasks_at_date_mut(date).collect(),
    )
    .prompt()?;

    TaskManager::complete_tasks(tasks_to_complete);

    Ok(())
}

fn delete_tasks_at_date(tm: &mut TaskManager) -> InquireResult<()> {
    let date = DateSelect::new("Date:").prompt()?;
    let tasks_to_delete =
        MultiSelect::new("Mark Tasks to Delete", tm.get_tasks_at_date(date).collect()).prompt()?;
    let task_id_to_delete: Vec<u32> = tasks_to_delete.iter().map(|task| task.id).collect();
    tm.delete_tasks(&task_id_to_delete);

    Ok(())
}

fn get_categories() -> Vec<&'static str> {
    vec![
        "Add Task",
        "List Tasks",
        "List Tasks At",
        "Complete Tasks At",
        "Delete Tasks At",
        "Quit",
    ]
}
