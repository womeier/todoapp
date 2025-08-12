use super::schema::tasks;
use crate::errors::AppError;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::*;

#[derive(Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: bool,
    pub label: String,
}

#[derive(Insertable, Deserialize, IntoParams, ToSchema)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub title: String,
    pub done: Option<bool>,
    pub label: Option<String>,
}

#[derive(Deserialize, AsChangeset, IntoParams, ToSchema)]
#[diesel(table_name = tasks)]
pub struct UpdateTask {
    pub done: Option<bool>,
    pub label: Option<String>,
}

pub fn label_valid(label: &str) -> bool {
    label.eq("Now") || label.eq("Watch") || label.eq("Later")
}

pub trait Validated {
    fn validate(&self) -> Result<(), AppError>;
}

impl Validated for UpdateTask {
    fn validate(&self) -> Result<(), AppError> {
        if !self.label.as_ref().is_none_or(|l| label_valid(l)) {
            return Err(AppError::BadJson("label not allowed".to_owned()));
        }

        Ok(())
    }
}

impl Validated for NewTask {
    fn validate(&self) -> Result<(), AppError> {
        if !self.label.as_ref().is_none_or(|l| label_valid(l)) {
            return Err(AppError::BadJson("label not allowed".to_owned()));
        }

        Ok(())
    }
}
