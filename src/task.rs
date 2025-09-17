use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
Todo,
InProgress,
Done,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Recurrence {
Once,
Daily,
Weekly,
Monthly,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
pub id: Uuid,
pub title: String,
pub description: Option<String>,
pub status: Status,
pub due_date: Option<NaiveDate>,
pub category: Option<String>,
pub recurrence: Recurrence,
pub created_at: DateTime<Utc>,
pub updated_at: DateTime<Utc>,
}


impl Task {
pub fn new(title: String, description: Option<String>, due_date: Option<NaiveDate>, category: Option<String>, recurrence: Recurrence) -> Self {
let now = Utc::now();
Task {
id: Uuid::new_v4(),
title,
description,
status: Status::Todo,
due_date,
category,
recurrence,
created_at: now,
updated_at: now,
}
}
}