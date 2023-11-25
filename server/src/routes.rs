use crate::database::{
    create_task_db, delete_task_db, get_task_db, get_tasks_db, update_all_tasks_db, update_task_db,
};
use crate::models::Task;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use sled::Tree;
use std::sync::Arc;

/// Create a new task. The database id will be automatically assigned.
#[post("/task", format = "json", data = "<task>")]
fn create_task(db: &State<Arc<Tree>>, task: Json<Task>) -> status::Accepted<String> {
    // Delegate the task creation to the create_task function
    create_task_db(db, task);

    status::Accepted("success".to_string())
}

/// Return all tasks or an empty Vec, which is valid.
#[get("/tasks")]
fn get_tasks(db: &State<Arc<Tree>>) -> Json<Vec<Task>> {
    // Call the get_tasks method to retrieve tasks
    get_tasks_db(db)
}

/// Update all tasks with a Vec<Task>.
#[post("/tasks", format = "application/json", data = "<tasks>")]
fn update_all_tasks(db: &State<Arc<Tree>>, tasks: Json<Vec<Task>>) -> status::Accepted<String> {
    update_all_tasks_db(db, tasks);

    status::Accepted("success".to_string())
}

/// Get a task by id.
#[get("/task/<id>")]
fn get_task(db: &State<Arc<Tree>>, id: u8) -> Option<Json<Task>> {
    get_task_db(db, id)
}

/// Update a task by id.
#[put("/task/<id>", format = "application/json", data = "<task>")]
fn update_task(db: &State<Arc<Tree>>, id: u8, task: Json<Task>) -> status::Accepted<String> {
    update_task_db(db, id, task);

    status::Accepted("Task was updated successfully!".to_string())
}

/// Delete a task by id.
#[delete("/task/<id>")]
fn delete_task(db: &State<Arc<Tree>>, id: u8) -> status::Accepted<String> {
    delete_task_db(db, id);

    status::Accepted("Task was deleted successfully!".to_string())
}

pub fn all_routes() -> Vec<rocket::Route> {
    routes![
        create_task,
        get_task,
        get_tasks,
        update_all_tasks,
        update_task,
        delete_task
    ]
}
