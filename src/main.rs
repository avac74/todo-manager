#[macro_use]
extern crate prettytable;
extern crate chrono;

mod status;
mod task;
mod tasklist;

use std::path::Path;
use structopt::StructOpt;
use todo_lib::todo;

use status::*;
use task::*;
use tasklist::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "todo-manager")]
enum TaskManCommands {
    #[structopt(name = "ls", about = "Lists tasks")]
    List {
        #[structopt(short = "c", long = "complete", help = "List only completed tasks")]
        complete: bool,

        #[structopt(short = "t", long = "todo", help = "List only non-completed tasks")]
        todo: bool,

        #[structopt(
            short = "n",
            long = "next",
            help = "List tasks that can be done next (non-completed, non-blocked)"
        )]
        next: bool,

        #[structopt(
            short = "f",
            long = "filter",
            help = "List tasks which contain the keyword in their description"
        )]
        keyword: Option<String>,

        #[structopt(name = "FILE", default_value = "todo.txt")]
        filename: String,
    },

    #[structopt(name = "complete", about = "Mark a task as complete by its ID")]
    Complete { id: String, filename: String },

    #[structopt(name = "clearall", about = "Mark all tasks as not complete")]
    ClearAll { filename: String },
}

fn main() {
    let opt = TaskManCommands::from_args();
    let mut task_vec: TaskList;

    match &opt {
        TaskManCommands::List {
            complete,
            todo,
            next,
            keyword,
            filename,
        } => {
            let path = Path::new(filename);
            if !path.exists() {
                println!("File {} does not exist", filename);
                return;
            }

            task_vec = mark_blocked_tasks(load_todo_into_tasks(filename));

            if *complete {
                task_vec = mark_blocked_tasks(
                    task_vec
                        .into_iter()
                        .filter(|task| task.is_complete())
                        .collect(),
                );
            }

            if *todo {
                task_vec = mark_blocked_tasks(
                    task_vec
                        .into_iter()
                        .filter(|task| !task.is_complete())
                        .collect(),
                );
            }

            if *next {
                task_vec = task_vec
                    .into_iter()
                    .filter(|x| !x.is_blocked && !x.is_complete())
                    .collect();
            }

            task_vec = match &*keyword {
                Some(x) => task_vec
                    .into_iter()
                    .filter(|y| y.subject.contains(x))
                    .collect(),
                None => task_vec,
            };

            print_task_table(&task_vec);
        }

        TaskManCommands::Complete { id, filename } => {
            task_vec = mark_blocked_tasks(load_todo_into_tasks(filename));

            let res: TaskList = task_vec
                .into_iter()
                .map(|task| {
                    if task.is_id(id.to_string()) {
                        Task {
                            status: Status::Complete,
                            ..task
                        }
                    } else {
                        task
                    }
                })
                .collect();

            task_vec = mark_blocked_tasks(res);

            print_task_table(&task_vec);

            let tv = todo_vec_from_task_vec(task_vec);
            todo::save(&tv[..], Path::new(filename)).unwrap();
        }

        TaskManCommands::ClearAll { filename } => {
            task_vec = load_todo_into_tasks(filename);

            let res: TaskList = task_vec
                .into_iter()
                .map(|task| Task {
                    status: Status::Todo,
                    ..task
                })
                .collect();

            task_vec = mark_blocked_tasks(res);

            print_task_table(&task_vec);

            let tv = todo_vec_from_task_vec(task_vec);
            todo::save(&tv[..], Path::new(filename)).unwrap();
        }
    };
}
