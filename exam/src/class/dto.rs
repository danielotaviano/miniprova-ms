use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddExamToClassDto {
    pub exam_id: i32,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateClassInputDto {
    pub name: String,
    pub code: String,
    pub description: String,
}

impl CreateClassInputDto {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Name is required".to_string());
        }

        if self.code.is_empty() {
            return Err("Code is required".to_string());
        }

        if self.description.is_empty() {
            return Err("Description is required".to_string());
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateClassInputDto {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl UpdateClassInputDto {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(name) = &self.name {
            if name.is_empty() {
                return Err("Name is required".to_string());
            }
        }

        if let Some(description) = &self.description {
            if description.is_empty() {
                return Err("Description is required".to_string());
            }
        }

        Ok(())
    }
}
