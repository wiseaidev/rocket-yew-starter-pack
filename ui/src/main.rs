use gloo::storage::LocalStorage;
use gloo_net::http::Request;
use gloo_storage::Storage;
use gloo_timers::callback::Timeout;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use std::collections::HashMap;

const KEY: &str = "yew.todomvc.self";

#[derive(Serialize, Deserialize)]
pub struct State {
    entries: Vec<Entry>,
    filter: Filter,
    value: String,
    edit_value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entry {
    description: String,
    completed: bool,
    editing: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    fn fit(&self, entry: &Entry) -> bool {
        match self {
            Filter::All => true,
            Filter::Active => !entry.completed,
            Filter::Completed => entry.completed,
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    let storage_data: HashMap<String, String> = LocalStorage::get(KEY).unwrap_or_default();
    let data_handle = use_state(|| storage_data);
    let data = (*data_handle).clone();
    let data_values = (*data_handle).clone();

    let entries = use_state(|| vec![]);
    let filter = use_state(|| Filter::All);
    let value = use_state(|| "".to_string());
    let edit_value = use_state(|| "".to_string());

    let _ = Timeout::new(5_000, move || {
        let entries = entries.clone();
        spawn_local(async move {
            let entries = entries.clone();
            let url = format!("http://[::]:8000/tasks");
            let fetched_entries: Vec<Entry> = Request::get(url.as_str())
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            entries.set(fetched_entries);
            log::info!("got data from server: {:?}", data);
            // TODO: Log error: log::info!("initial sync failed");
        });
    });

    let view_input = {
        let input_key_ref = input_key_ref.clone();
        let input_key_handle = input_key_handle.clone();

        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let new_entry = Entry {
                    description: *value.clone(),
                    completed: false,
                    editing: false,
                };
                entries_handle.set(entries.clone().push(new_entry));
                value.set("".to_string());
            }
        });
    };

    let view_filter = |flt: Filter| {
        html! {
            <li>
                <a class={if filter == flt { "selected" } else { "not-selected" }},
                   href=&flt,
                   onclick=|_| set_filter(flt.clone())>
                    { flt }
                </a>
            </li>
        }
    };

    let view_entry_edit_input = |idx: usize, entry: &Entry| {
        if entry.editing {
            html! {
                <input class="edit"
                       type="text"
                       value=&entry.description
                       oninput=|e| set_edit_value(e.value())
                       onblur=|_| {
                           let updated_entries = entries.iter().enumerate().map(|(i, e)| {
                               if i == idx {
                                   Entry {
                                       description: edit_value.clone(),
                                       editing: false,
                                       ..e.clone()
                                   }
                               } else {
                                   e.clone()
                               }
                           }).collect();
                           set_entries(updated_entries);
                           set_edit_value("".to_string());
                       }
                       onkeypress=|e| {
                           if e.key() == "Enter" {
                               let updated_entries = entries.iter().enumerate().map(|(i, e)| {
                                   if i == idx {
                                       Entry {
                                           description: edit_value.clone(),
                                           editing: false,
                                           ..e.clone()
                                       }
                                   } else {
                                       e.clone()
                                   }
                               }).collect();
                               set_entries(updated_entries);
                               set_edit_value("".to_string());
                           }
                       } />
            }
        } else {
            html! {
                <input type="hidden" />
            }
        }
    };

    let view_entry = |(idx, entry): (usize, &Entry)| {
        html! {
            <li class=if entry.editing { "editing" } else { "" }>
                <div class="view">
                    <input class="toggle" type="checkbox" checked=entry.completed onclick=|_| {
                        let updated_entries = entries.iter().enumerate().map(|(i, e)| {
                            if i == idx {
                                Entry {
                                    completed: !entry.completed,
                                    ..e.clone()
                                }
                            } else {
                                e.clone()
                            }
                        }).collect();
                        set_entries(updated_entries);
                    } />
                    <label ondoubleclick=|_| {
                        set_edit_value(entry.description.clone());
                        let updated_entries = entries.iter().enumerate().map(|(i, e)| {
                            if i == idx {
                                Entry {
                                    editing: !entry.editing,
                                    ..e.clone()
                                }
                            } else {
                                e.clone()
                            }
                        }).collect();
                        set_entries(updated_entries);
                    }>{ &entry.description }</label>
                    <button class="destroy" onclick=|_| {
                        let updated_entries = entries.iter().enumerate().filter_map(|(i, e)| {
                            if i == idx {
                                None
                            } else {
                                Some(e.clone())
                            }
                        }).collect();
                        set_entries(updated_entries);
                    } />
                </div>
                { view_entry_edit_input(idx, entry) }
            </li>
        }
    };

    // Render the main HTML structure
    html! {
        <div class="todomvc-wrapper">
            <section class="todoapp">
                <header class="header">
                    <h1>{ "todos" }</h1>
                    <input class="new-todo",
                           placeholder="What needs to be done?",
                           value=value,
                           oninput=|e| set_value(e.value()),
                           onkeypress=view_input />
                </header>
                <section class="main">
                    <input class="toggle-all" type="checkbox" checked=entries.iter().all(|entry| entry.completed) onclick=|_| {
                        let updated_entries = entries.iter().map(|entry| Entry {
                            completed: !entries.iter().all(|entry| entry.completed),
                            ..entry.clone()
                        }).collect();
                        set_entries(updated_entries);
                    } />
                    <ul class="todo-list">
                        { for entries.iter().filter(|e| filter.fit(e)).enumerate().map(view_entry) }
                    </ul>
                </section>
                <footer class "footer">
                    <span class="todo-count">
                        <strong>{ entries.len() }</strong>
                        { " item(s) left" }
                    </span>
                    <ul class="filters">
                        { for Filter::iter().map(view_filter) }
                    </ul>
                    <button class="clear-completed" onclick=|_| {
                        let updated_entries = entries.iter().filter(|e| !e.completed).cloned().collect();
                        set_entries(updated_entries);
                    }>
                        { format!("Clear completed ({})", entries.iter().filter(|e| e.completed).count()) }
                    </button>
                </footer>
            </section>
            <footer class="info">
                <p>{ "Double-click to edit a todo" }</p>
                <p>{ "Written by " }<a href="https://github.com/DenisKolodin/" target="_blank">{ "Denis Kolodin" }</a></p>
                <p>{ "Part of " }<a href="http://todomvc.com/" target="_blank">{ "TodoMVC" }</a></p>
            </footer>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
