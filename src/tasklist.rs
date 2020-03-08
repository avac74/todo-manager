use crate::status::*;
use crate::task::*;
use prettytable::{format, Table};
use std::path::Path;
use todo_lib::todo;

pub type TaskList = Vec<Task>;

#[allow(dead_code)]
pub fn print_task_list(tasks: &TaskList) {
    for task in tasks {
        println!("{}", task);
    }
}

pub fn print_task_table(tasks: &TaskList) {
    if tasks.len() > 0 {
        let mut table = Table::new();

        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        table.set_titles(row![
            "ID",
            " Status",
            "Priority",
            "Description",
            "Projects",
            "Blocked By",
            "Contexts",
            "Due Date"
        ]);
        for task in tasks {
            if task.is_blocked {
                table.add_row(row![
                          task.get_id(),
                          Fyc -> task.status.to_string(),
                          Fbc -> task.priority.to_string(),
                          Fr -> task.subject.to_string(),
                          Fc -> task.projects.join(","),
                          Fr -> task.get_after_as_csv(),
                          task.contexts.join(","),
                          Fm -> format!("{}{}", task.get_due_date(), if task.is_task_due() { " ⚠" } else { "" }),
            ]);
            } else {
                table.add_row(row![
                          task.get_id(),
                          Fyc -> task.status.to_string(),
                          Fbc -> task.priority.to_string(),
                          Fg -> task.subject.to_string(),
                          Fc -> task.projects.join(","),
                          Fr -> task.get_after_as_csv(),
                          task.contexts.join(","),
                          Fm -> format!("{}{}", task.get_due_date(), if task.is_task_due() { " ⚠" } else { "" }),
            ]);
            }
        }

        table.printstd();
    } else {
        print!("There are no tasks to display");
    }
}

pub fn find_task_by_id<'a>(id: &'a str, all_tasks: &'a TaskList) -> Option<&'a Task> {
    let mut iter = all_tasks.iter();

    iter.find(|&x| match &x.id {
        Some(y) => y == id,
        None => false,
    })
}

pub fn is_task_complete_by_id(id: &str, all_tasks: &TaskList) -> bool {
    let res = find_task_by_id(id, all_tasks);
    match res {
        Some(x) => x.is_complete(),
        _ => true,
    }
}

pub fn load_todo_into_tasks(filename: &str) -> TaskList {
    let todo_vec = todo::load(Path::new(filename));

    let mut task_vec: TaskList = Vec::new();

    match todo_vec {
        Ok(tasks) => {
            for task in tasks {
                task_vec.push(Task {
                    priority: task.inner.priority,
                    subject: task.inner.subject,
                    projects: task.inner.projects,
                    contexts: task.inner.contexts,
                    id: match task.inner.tags.get("id") {
                        Some(x) => Some(x.to_string()),
                        None => None,
                    },
                    before: Vec::new(),
                    after: match task.inner.tags.get("after") {
                        Some(x) => x.split(",").map(|x| String::from(x)).collect(),
                        None => Vec::new(),
                    },
                    is_blocked: false,
                    due: task.inner.due_date,
                    status: if task.inner.finished {
                        Status::Complete
                    } else {
                        Status::Todo
                    },
                });
            }
        }
        Err(_) => {
            panic!("Oh damn, something went wrong!");
        }
    }

    task_vec
}

pub fn mark_blocked_tasks(mut all_tasks: TaskList) -> TaskList {
    // Creates a set with all blocking IDs
    let mut blockers: std::collections::HashSet<String> = std::collections::HashSet::new();
    for task in &all_tasks {
        if task.after.len() > 0 {
            for after in &task.after {
                if !is_task_complete_by_id(&after, &all_tasks) {
                    blockers.insert(after.to_string());
                }
            }
        }
    }

    for task in &mut all_tasks {
        task.is_blocked = false;
        for after in &task.after {
            if blockers.contains(after) {
                task.is_blocked = true;
            }
        }
    }

    all_tasks
}

pub fn todo_vec_from_task_vec(all_tasks: TaskList) -> Vec<todo_txt::task::Extended> {
    let mut res: Vec<todo_txt::task::Extended> = Vec::new();

    for task in all_tasks {
        res.push(task.to_todo());
    }

    res
}
