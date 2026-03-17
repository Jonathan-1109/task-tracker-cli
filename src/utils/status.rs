use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize,  Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum StatusTask {
    Pending,
    InProgress,
	Completed
}

impl fmt::Display for StatusTask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StatusTask::Pending => write!(f, "Pendiente"),
            StatusTask::InProgress => write!(f, "En progreso"),
            StatusTask::Completed => write!(f, "Completado"),
        }
    }
    
}
impl StatusTask {
    pub fn next(&self) -> StatusTask{
        match self {
            StatusTask::Pending => StatusTask::InProgress,
            StatusTask::InProgress => StatusTask::Completed,
            StatusTask::Completed => StatusTask::Completed
        }
    }
    pub fn compare(&self) -> String {
        match self {
            StatusTask::Pending => return "pending".to_string(),
            StatusTask::InProgress => return "inprogress".to_string(),
            StatusTask::Completed => return "completed".to_string(),
        };
    }
}