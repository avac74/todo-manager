use crate::status::*;
use ansi_term::Colour;
use chrono::Datelike;

#[derive(Debug)]
pub struct Task {
    pub priority: u8,
    pub subject: String,
    pub status: Status,
    pub projects: Vec<String>,
    pub contexts: Vec<String>,
    pub id: Option<String>,
    pub before: Vec<String>,
    pub after: Vec<String>,
    pub due: Option<chrono::NaiveDate>,
    pub is_blocked: bool,
}

impl Task {
    pub fn is_complete(&self) -> bool {
        match self.status {
            Status::Complete => true,
            _ => false,
        }
    }

    pub fn is_id(&self, id: String) -> bool {
        match &self.id {
            Some(x) => x == &id,
            None => false,
        }
    }

    pub fn get_id(&self) -> String {
        match &self.id {
            Some(x) => x.to_string(),
            None => "".to_string(),
        }
    }

    pub fn get_due_date(&self) -> String {
        match &self.due {
            Some(x) => x.to_string(),
            None    => "".to_string(),
        }
    }

    pub fn is_task_due(&self) -> bool {
        match self.due {
            Some(x) => now_to_naive_date() >= x,
            None    => false,
        }
    }

    pub fn get_after_as_csv(&self) -> String {
        if self.after.len() > 0 {
            self.after.join(",")
        } else {
            "-".to_string()
        }
    }

    pub fn to_todo(&self) -> todo_txt::task::Extended {
        let mut tags: std::collections::BTreeMap<String, String> =
            std::collections::BTreeMap::new();

        match &self.id {
            Some(x) => {
                tags.insert("id".to_string(), x.to_string());
            }
            None => {}
        }

        if self.after.len() > 0 {
            tags.insert("after".to_string(), self.after.join(","));
        }

        todo_txt::task::Extended {
            inner: todo_txt::task::Task {
                finished: self.is_complete(),
                subject: self.subject[..].to_string(),
                priority: self.priority,
                projects: self.projects.as_slice().to_vec(),
                contexts: self.contexts.as_slice().to_vec(),
                tags: tags,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}",
            Colour::White.paint(format!(
                "{:<20}",
                match &self.id {
                    Some(x) => x,
                    None => "-",
                }
            )),
            Colour::Yellow.paint(format!("{:<5}", self.status.to_string())),
            Colour::Blue.paint(format!("{:<5}", self.priority.to_string())),
            if self.is_blocked {
                Colour::Red.paint(format!("{:<100}", self.subject.to_string()))
            } else {
                Colour::Green.paint(format!("{:<100}", self.subject.to_string()))
            },
            Colour::Cyan.paint(format!("{:<20}", self.projects.join(","))),
            if self.after.len() > 0 {
                Colour::Red.paint(format!("{:<40}", self.after.join(",")))
            } else {
                Colour::Red.paint(format!("{:<40}", "-"))
            },
            Colour::RGB(80, 80, 80).paint(format!("{:<20}", self.contexts.join(","))),
        )
    }
}

pub fn now_to_naive_date() -> chrono::NaiveDate {
    let local = chrono::prelude::Local::now();

    chrono::NaiveDate::from_ymd(local.year(), local.month(), local.day())
}
