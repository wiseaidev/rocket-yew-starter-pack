#[macro_use]
extern crate rocket;

use bincode::{deserialize, serialize};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use sled::Mode::LowSpace;
use sled::{Config, Tree};
use std::sync::Arc;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket::http::Method;
use std::path::PathBuf;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let allowed_origins = AllowedOrigins::all();
    let allowed_methods = vec![Method::Get, Method::Post, Method::Options, Method::Put].into_iter().map(From::from).collect();
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: allowed_methods,
        allowed_headers: AllowedHeaders::all(),
        ..rocket_cors::CorsOptions::default()
    }
    .to_cors()
    .expect("Error in CORS setup");
    let path = "data.db";
    let config = Config::new()
        .path(path)
        .mode(LowSpace)
        .cache_capacity(1_000_000)
        .flush_every_ms(Some(1000));
    let tree: Tree = config.open().unwrap().open_tree("tree").unwrap();
    let db_arc = Arc::new(tree);
    let routes = all_routes();
    rocket::build()
        .mount("/", routes)
        .attach(cors)
        .manage(db_arc)
        .launch()
        .await
        .expect("Launch Error");
    Ok(())
}

fn all_routes() -> Vec<rocket::Route> {
    routes![
        create_task,
        get_task,
        get_tasks,
        update_all_tasks,
        update_task
    ]
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    completed: bool,
    description: String,
    editing: bool,
}

/// Create a new task. The database id will be automatically assigned.
#[post("/task", format = "json", data = "<task>")]
async fn create_task(db: &State<Arc<sled::Tree>>, task: Json<Task>) -> status::Accepted<String> {
    println!("got a task {:?}", task);

    // scan through our DB to get create an incremented ID.
    let mut count = 0;
    for item in db.iter() {
        if item.is_ok() {
            count += 1;
        }
    }

    // Keys and Values in sled are Vec<u8>
    let new_key = vec![count];

    // Our task is the first field (e.g., "0") on Json<Task> Rocket passes us.
    let encoded: Vec<u8> = serialize(&task.0).unwrap();
    let _ = db.insert(new_key, encoded);
    status::Accepted(Some(format!("success")))
}

/// Return all tasks or an empty Vec, which is valid.
#[get("/tasks")]
fn get_tasks(db: &State<Arc<sled::Tree>>) -> Json<Vec<Task>> {
    let mut results: Vec<Task> = Vec::new();

    for item in db.iter() {
        match item {
            Ok((_, v)) => {
                let decoded: Task = deserialize(&v[..]).expect("could not deserialize Task");
                results.push(decoded);
            }
            _ => {}
        }
    }

    Json(results)
}

/// Update all tasks with a Vec<Task>.
#[post("/tasks", format = "application/json", data = "<tasks>")]
fn update_all_tasks(
    db: &State<Arc<sled::Tree>>,
    tasks: Json<Vec<Task>>,
) -> status::Accepted<String> {
    // get len
    let mut count = 0;
    for item in db.iter() {
        match item {
            Ok(_) => count += 1,
            _ => {}
        }
    }

    // delete everything
    for k in 0..count {
        db.remove(&vec![k as u8]).expect("delete failed");
    }

    // update everything
    for (i, ref v) in tasks.0.into_iter().enumerate() {
        let encoded: Vec<u8> = serialize(v).unwrap();
        let key = vec![i as u8];
        let _ = db.insert(key, encoded);
    }

    status::Accepted(Some(format!("success")))
}

/// Get a task by id.
#[get("/task/<id>")]
fn get_task(db: &State<Arc<sled::Tree>>, id: u8) -> Option<Json<Task>> {
    let val = db.get(&vec![id]);
    match val {
        Ok(Some(db_vec)) => {
            let decoded: Task = deserialize(&db_vec[..]).expect("unable to decode Task");
            Some(Json(decoded))
        }
        _ => None,
    }
}

/// Update a task by id.
#[put("/task/<id>", format = "application/json", data = "<task>")]
fn update_task(db: &State<Arc<sled::Tree>>, id: u8, task: Json<Task>) -> status::Accepted<String> {
    let key = vec![id];
    let encoded: Vec<u8> = serialize(&task.0).unwrap();
    let _ = db.insert(key, encoded);

    status::Accepted(Some(format!("format")))
}

/// Create an instance of Rocket suitable for tests.
fn _test_instance(db_path: PathBuf) -> rocket::Rocket<rocket::Build> {
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
fn test_post_get() {
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;
    use tempdir::TempDir;

    let dir = TempDir::new("rocket").unwrap();
    let path = dir.path().join("test_data.db");

    // create our test client
    let c = Client::tracked(_test_instance(path)).expect("valid rocket");

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
