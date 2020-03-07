#[derive(Debug)]
pub enum Status {
    Complete,
    Todo
}

impl std::string::ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Complete => String::from("(✔)"),
            Status::Todo => String::from("( )"),
        }
    }
}
