use crate::errors::AppError;
use crate::models::*;
use crate::schema::tasks;
use crate::schema::tasks::dsl::*;
use crate::*;
use axum::{extract::Path, extract::State, http::StatusCode, response::IntoResponse, Json};
use diesel::prelude::*;
use tracing::debug;
use utoipa;

#[utoipa::path(get, path = "/tasks", responses((status = OK, body = [Task])))]
pub async fn list_tasks(State(config): State<ServerConfig>) -> Result<impl IntoResponse, AppError> {
    debug!("GET all");
    let connection = &mut establish_connection(&config.db_path);

    let results = tasks
        .select(Task::as_select())
        .filter(done.eq(false))
        .load(connection)
        .map_err(AppError::DbError)?;

    Ok((StatusCode::OK, Json(results)))
}

#[utoipa::path(get, path = "/tasks/{tid}", responses((status = OK, body = Task)))]
pub async fn get_task(
    State(config): State<ServerConfig>,
    Path(tid): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    debug!("GET {tid:?}");
    let connection = &mut establish_connection(&config.db_path);

    let task = tasks
        .find(tid)
        .select(Task::as_select())
        .first(connection)
        .or(Err(AppError::TaskNotFound))?;

    Ok(Json(task))
}

#[utoipa::path(post, path = "/tasks", params(NewTask), responses((status = CREATED, body = Task)))]
pub async fn create_task(
    State(config): State<ServerConfig>,
    Json(payload): Json<NewTask>,
) -> Result<impl IntoResponse, AppError> {
    debug!("POST title=\"{}\"", payload.title);
    let connection = &mut establish_connection(&config.db_path);

    payload.validate()?;

    let new_task = NewTask {
        title: payload.title.to_string(),
        done: payload.done.or(Some(false)),
        label: payload.label.or(Some("Now".to_owned())),
    };

    let task = diesel::insert_into(tasks::table)
        .values(&new_task)
        .returning(Task::as_returning())
        .get_result(connection)
        .map_err(errors::AppError::DbError)?;

    Ok((StatusCode::CREATED, Json(task)))
}

#[utoipa::path(post, path = "/tasks/{tid}", params(UpdateTask), responses((status = OK)))]
pub async fn update_task(
    State(config): State<ServerConfig>,
    Path(tid): Path<i32>,
    Json(payload): Json<UpdateTask>,
) -> Result<impl IntoResponse, AppError> {
    debug!("POST id=\"{tid}\"");

    payload.validate()?;

    let connection = &mut establish_connection(&config.db_path);

    diesel::update(tasks.find(tid))
        .set(&payload)
        .execute(connection)
        .map_err(AppError::DbError)?;

    Ok(StatusCode::OK)
}
