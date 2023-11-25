use crate::models::Task;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::{from_str, to_string, to_vec};
use sled::Mode::LowSpace;
use sled::{Config, Tree};
use std::path::PathBuf;
use std::sync::Arc;

pub fn create_task_db(db: &State<Arc<Tree>>, task: Json<Task>) {
    // Get the maximum id in the database and increment by 1 for the new id
    let next_id = match db
        .iter()
        .filter_map(|item| item.ok().map(|(key, _)| key.last().cloned()))
        .max()
        .map(|max_id| max_id.map(|id| id + 1).unwrap_or(0))
    {
        Some(id) => id,
        None => 0, // If the database is empty, start with id 0
    };

    // Convert the next_id to u8
    let next_id_u8 = match next_id {
        id if id <= u8::MAX => id as u8,
        _ => {
            eprintln!("Error: Next id exceeds the maximum value for u8");
            return;
        }
    };

    // Create a new key as a vector containing the next id
    let new_key = vec![next_id_u8];

    // Serialize the task into a JSON string
    let encoded = match to_string(&task.0) {
        Ok(encoded) => encoded,
        Err(err) => {
            eprintln!("Error serializing task: {:?}", err);
            return;
        }
    };

    // Insert the new task into the database
    if let Err(err) = db.insert(new_key.clone(), encoded.as_bytes()) {
        eprintln!("Error inserting task into the database: {:?}", err);
    } else {
        println!("Task inserted successfully with key: {:?}", new_key);
    }
}

pub fn get_tasks_db(db: &State<Arc<Tree>>) -> Json<Vec<Task>> {
    // Vector to store successfully deserialized tasks
    let results: Vec<Task> = db
        .iter()
        .filter_map(|item| {
            match item {
                // Successfully retrieved an item from the database
                Ok((k, v)) => {
                    match from_str::<Task>(&String::from_utf8_lossy(&v)) {
                        // Successfully deserialized the JSON into a Task
                        Ok(mut decoded) => {
                            // Convert the Vec<u8> key to a u8
                            let id = k.last().cloned().unwrap_or_default();
                            decoded.id = id;
                            Some(decoded)
                        }
                        // Handle deserialization error
                        Err(err) => {
                            // Print error message to stderr
                            eprintln!("Error deserializing Task: {:?}", err);
                            // Return None to filter out this item
                            None
                        }
                    }
                }
                // Handle error during iteration over database items
                Err(err) => {
                    // Print error message to stderr
                    eprintln!("Error iterating over database items: {:?}", err);
                    // Return None to filter out this item
                    None
                }
            }
        })
        .collect();

    // Return the successfully deserialized tasks as JSON
    Json(results)
}

/// Get a task by id from the database.
///
/// # Arguments
///
/// * `db` - The database state containing the tasks.
/// * `id` - The id of the task to retrieve.
///
/// # Returns
///
/// Returns an `Option<Json<Task>>` representing the retrieved task if successful,
/// or `None` if the task is not found or an error occurs.
pub fn get_task_db(db: &State<Arc<Tree>>, id: u8) -> Option<Json<Task>> {
    // Retrieve the task from the database based on the provided id
    let val = match db.get(&vec![id]) {
        Ok(Some(db_vec)) => db_vec,
        Ok(None) => {
            // Task not found in the database
            println!("Task with id {} not found in the database.", id);
            return None;
        }
        Err(err) => {
            // Handle error retrieving task from the database
            eprintln!("Error retrieving task from the database: {:?}", err);
            return None;
        }
    };

    // Deserialize the retrieved value into a Task
    match from_str::<Task>(&String::from_utf8_lossy(&val)) {
        // Successfully deserialized the JSON into a Task
        Ok(mut decoded) => {
            // Set the id field in Task using the provided id
            decoded.id = id;
            Some(Json(decoded))
        }
        Err(err) => {
            // Handle error decoding Task
            eprintln!("Error decoding Task: {:?}", err);
            None
        }
    }
}

/// Update a task by id.
pub fn update_task_db(db: &State<Arc<Tree>>, id: u8, task: Json<Task>) {
    // Create a key using the provided id
    let key = vec![id];

    // Serialize the task into a Vec<u8>
    let encoded = match to_vec(&task.0) {
        Ok(encoded) => encoded,
        Err(err) => {
            eprintln!("Error serializing task: {:?}", err);
            return;
        }
    };

    // Insert the updated task into the database
    if let Err(err) = db.insert(key.clone(), encoded) {
        eprintln!("Error updating task in the database: {:?}", err);
    } else {
        println!("Task with id {} updated successfully.", id);
    }
}

/// Delete a task by id.
pub fn delete_task_db(db: &State<Arc<Tree>>, id: u8) {
    // Create a key using the provided id
    let key = vec![id];

    // Remove the task from the database
    match db.remove(key.clone()) {
        Ok(_) => {
            println!("Task with id {} deleted successfully.", id);
        }
        Err(err) => {
            eprintln!("Error deleting task from the database: {:?}", err);
        }
    }
}

/// Update all tasks with a Vec<Task>.
pub fn update_all_tasks_db(db: &State<Arc<Tree>>, tasks: Json<Vec<Task>>) {
    // Get the current count of items in the database
    let count = db.iter().filter(Result::is_ok).count();
    let count_u8 = match count {
        count if count <= u8::MAX as usize => count as u8,
        _ => {
            eprintln!("Error: Count exceeds the maximum value for u8");
            return;
        }
    };

    // Delete everything in the database
    for k in 0..count_u8 {
        if let Err(err) = db.remove(&vec![k as u8]) {
            eprintln!("Error deleting item from the database: {:?}", err);
        }
    }

    // Update everything in the database
    for (i, task) in tasks.0.into_iter().enumerate() {
        // Serialize the task into a Vec<u8>
        let encoded = match to_vec(&task) {
            Ok(encoded) => encoded,
            Err(err) => {
                eprintln!("Error serializing task: {:?}", err);
                continue; // Skip this task and move to the next one
            }
        };

        // Insert the serialized task into the database
        let key = vec![i as u8];
        if let Err(err) = db.insert(key.clone(), encoded) {
            eprintln!("Error inserting task into the database: {:?}", err);
        } else {
            println!("Task inserted successfully with key: {:?}", key);
        }
    }
}

/// Set up a sled database and return an Arc<Tree>.
pub fn setup_database(path: PathBuf) -> Arc<Tree> {
    // Configure sled with the provided path and options
    let config = Config::new()
        .path(path)
        .mode(LowSpace)
        .cache_capacity(1_000_000)
        .flush_every_ms(Some(1000));

    // Open the tree within the database
    let tree = match config.open() {
        Ok(db) => match db.open_tree("tree") {
            Ok(tree) => tree,
            Err(err) => {
                eprintln!("Error opening tree in the database: {:?}", err);
                std::process::exit(1);
            }
        },
        Err(err) => {
            eprintln!("Error opening database: {:?}", err);
            std::process::exit(1);
        }
    };

    // Wrap the tree in an Arc for shared ownership
    Arc::new(tree)
}
