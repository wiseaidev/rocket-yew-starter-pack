#[macro_use]
extern crate rocket;

use sled::Mode::LowSpace;
use sled::{Config, Tree};
use std::path::PathBuf;
use std::sync::Arc;

use server::{all_routes, Task};

/// Create an instance of Rocket suitable for tests.
fn test_instance(db_path: PathBuf) -> rocket::Rocket<rocket::Build> {
    let config = Config::new()
        .path(String::from(db_path.to_str().unwrap()))
        .mode(LowSpace)
        .cache_capacity(1_000_000)
        .flush_every_ms(Some(1000));
    let tree: Tree = config.open().unwrap().open_tree("tree").unwrap();
    let db_arc = Arc::new(tree);
    rocket::build().mount("/", all_routes()).manage(db_arc)
}

#[test]
fn test_routes() {
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;
    use tempdir::TempDir;

    let dir = TempDir::new("rocket").unwrap();
    let path = dir.path().join("test_data.db");

    // create our test client
    let c = Client::tracked(test_instance(path)).expect("valid rocket");

    // create a new task with raw json string body
    let req = c
        .post("/task")
        .body(r#"{"completed": false, "description": "foo", "editing": false}"#)
        .header(ContentType::JSON);
    let resp = req.dispatch();
    assert_eq!(resp.status(), Status::Accepted);

    let req = c.get("/task/0");
    let bod = req.dispatch().into_bytes().unwrap();
    let decoded: Task = serde_json::from_slice(&bod[..])
        .expect("not a valid task; if your model has changed, try deleting your database file");
    assert_eq!(&decoded.description, "foo");

    // create another Task and let serde_json handle serialization
    let task = Task {
        description: String::from("baz"),
        completed: true,
        editing: false,
    };
    let req = c
        .post("/task")
        .body(serde_json::to_vec(&task).unwrap())
        .header(ContentType::JSON);
    let resp = req.dispatch();
    assert_eq!(resp.status(), Status::Accepted);

    // we expect our next task to have id 1
    let req = c.get("/task/1");
    let bod = req.dispatch().into_bytes().unwrap();
    let decoded: Task = serde_json::from_slice(&bod[..]).expect("not a valid task");
    assert_eq!(decoded.description, "baz");
    assert_eq!(decoded.completed, true);

    // now fetch both tasks from /tasks
    let req = c.get("/tasks");
    let bod = req.dispatch().into_bytes().unwrap();
    let tasks: Vec<Task> = serde_json::from_slice(&bod[..]).expect("not an array of Task");
    assert_eq!(tasks.len(), 2);

    // Test that they come back in the order we expect, with the data we expect.
    let foo_task = tasks.get(0).unwrap();
    let baz_task = tasks.get(1).unwrap();
    assert_eq!(foo_task.description, "foo");
    assert_eq!(baz_task.description, "baz");
}
