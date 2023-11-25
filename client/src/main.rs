use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

const BASE_URL: &str = "http://127.0.0.1:8000";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    id: u8,
    completed: bool,
    description: String,
    editing: bool,
}

#[function_component(CrudItems)]
fn crud_items() -> Html {
    let input_description_ref = use_node_ref();
    let input_description_handle = use_state(String::default);
    let input_description = (*input_description_handle).clone();

    let on_change = {
        let input_description_ref = input_description_ref.clone();
        let input_description_handle = input_description_handle.clone();

        Callback::from(move |_| {
            let input = input_description_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_description_handle.set(value);
            }
        })
    };

    // State to hold the new item's name
    let input_completed_ref = use_node_ref();
    let input_completed_handle = use_state(|| false);
    let input_completed = (*input_completed_handle).clone();

    let on_toggle = {
        let input_completed_ref = input_completed_ref.clone();
        let input_completed_handle = input_completed_handle.clone();

        Callback::from(move |_| {
            let input = input_completed_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                let is_checked = value == "on";
                input_completed_handle.set(is_checked);
            }
        })
    };

    let items = use_state(|| vec![]);
    let items_handle = items.clone();

    let updated_item = use_state(|| Item {
        id: 0,
        completed: false,
        description: "".to_string(),
        editing: false,
    });

    let on_fetch_items = {
        let items = items.clone();
        Callback::from(move |_| {
            let items = items.clone();
            spawn_local(async move {
                let fetched_items: Vec<Item> = Request::get(&format!("{}/tasks", BASE_URL))
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                items.set(fetched_items);
            });
        })
    };

    let on_submit = {
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let input_description = input_description.clone();
            let input_description_handle = input_description_handle.clone();
            let input_completed = input_completed.clone();
            spawn_local(async move {
                let item = Item {
                    id: 0,
                    completed: input_completed,
                    description: input_description,
                    editing: false,
                };
                let json_string = serde_json::to_string(&item)
                    .expect("Error while serializing JsValue to a string");

                match Request::post(&format!("{}/task", BASE_URL))
                    .header("Content-Type", "application/json")
                    .body(json_string)
                    .expect("Error while serializing the request body!")
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status() == 200 {
                            input_description_handle.set(String::new());
                        } else {
                        }
                    }
                    Err(error) => {
                        println!("Network request error: {:?}", error);
                    }
                }
            });
        })
    };

    let on_update_item = Callback::from(move |id: u64| {
        // Use the 'id' parameter to identify the item being updated
        let item_id = id;
        let updated_item = updated_item.clone();
        spawn_local(async move {
            let item: Item = Request::get(&format!("{}/task/{}", BASE_URL, item_id.clone()))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            updated_item.set(Item {
                id: item.id,
                completed: !item.completed,
                description: item.description,
                editing: item.editing,
            });
            let json_string = serde_json::to_string(&*updated_item)
                .expect("Error while serializing JsValue to a string");

            // Send a PUT request to update the item's completed status
            match Request::put(&format!("{}/task/{}", BASE_URL, item_id))
                .header("Content-Type", "application/json")
                .body(json_string)
                .expect("Error while serializing the request body!")
                .send()
                .await
            {
                Ok(response) => {
                    if response.status() == 200 {
                    } else {
                    }
                }
                Err(error) => {
                    // Handle the error here
                    println!("Network request error: {:?}", error);
                }
            }
        });
    });

    let on_delete_item = Callback::from(move |id: u64| {
        // Use the 'id' parameter to identify the item being deleted
        let item_id = id;
        spawn_local(async move {
            // Send a Delete request to update the item's completed status
            match Request::delete(&format!("{}/task/{}", BASE_URL, item_id))
                .header("Content-Type", "application/json")
                .send()
                .await
            {
                Ok(response) => {
                    if response.status() == 200 {
                    } else {
                    }
                }
                Err(error) => {
                    // Handle the error here
                    println!("Network request error: {:?}", error);
                }
            }
        });
    });

    use_effect_with(items.clone(), move |_| {
        // Fetch items on page refresh
        let items_handle = items_handle.clone();
        spawn_local(async move {
            let fetched_items: Vec<Item> = Request::get(&format!("{}/tasks", BASE_URL))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            items_handle.set(fetched_items);
        });

        Box::new(move || {
            // Cleanup logic
        }) as Box<dyn FnOnce()>
    });

    html! {
        <div class="container">
            <div class="split-screen">
                <div class="left-section">
                    <h2>{"Items Created"}</h2>
                    <button onclick={on_fetch_items}>{"Refresh Items"}</button>
                    <ul>
                        { for items.iter().enumerate().map(|(_, item)| render_item(item.id.try_into().unwrap(), item, on_update_item.clone(), on_delete_item.clone())) }
                    </ul>
                </div>
                <form class="form-container" onsubmit={on_submit}>
                    <div class="input-group">
                        <input
                            type="text"
                            id="item-description"
                            name="item-description"
                            placeholder="Item description"
                            required={true}
                            ref={input_description_ref}
                            oninput={on_change}
                        />
                    </div>

                    <div class="input-group">
                        <input
                            type="checkbox"
                            id="item-completed"
                            name="item-completed"
                            ref={input_completed_ref}
                            onclick={on_toggle}
                        />
                        <label for="item-completed">{"Mark as Done"}</label>
                    </div>

                    <div class="button-container">
                        <button type="submit">{ "Add Item" }</button>
                    </div>
                </form>
            </div>
        </div>
    }
}

fn render_item(
    index: u64,
    item: &Item,
    on_update_item: Callback<u64>,
    on_delete_item: Callback<u64>,
) -> Html {
    html! {
        <li class={if item.completed { "completed" } else { "" }}>
            <span>
                <strong>{format!("ID - {:?} -  ", index)}</strong>
                {&item.description}
                {if item.completed { " (Completed)" } else { " (Not Completed)" }}
            </span>
            <input
                type="checkbox"
                id="item-completed"
                name="item-completed"
                checked={item.completed}
                onclick={Callback::from(move |event: MouseEvent| {
            event.prevent_default(); on_update_item.emit(index)})}
            />
            <button onclick={Callback::from(move |event: MouseEvent| {
            event.prevent_default(); on_delete_item.emit(index)})}>{"Delete"}</button>
        </li>
    }
}

fn main() {
    yew::Renderer::<CrudItems>::new().render();
}
