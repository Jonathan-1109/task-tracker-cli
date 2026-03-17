use serde::{Deserialize, Serialize};

use crate::utils::status::StatusTask;

#[derive(Serialize,  Deserialize, Clone, PartialEq, Debug)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub status: StatusTask,
    pub create: String,
    pub update: String
}

#[derive(Serialize, Deserialize)]
pub struct NextID {
    pub next: u32
}
