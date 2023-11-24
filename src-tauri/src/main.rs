// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::borrow::BorrowMut;

use glob::glob;
use tauri::{api::dialog, CustomMenuItem, Menu, MenuItem, Submenu};

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn get_files(p: &std::path::PathBuf) {
    let source_file_glob = p.to_str().unwrap();
    for entry in glob(format!("{}/{}", source_file_glob, "*.mp3").as_str()).unwrap() {
        match entry {
            Ok(path) => println!("{}", path.display()),
            Err(e) => println!("{}", e),
        }
    }
}

fn main() {
    let file_menu = Submenu::new(
        "File",
        Menu::new()
            .add_item(CustomMenuItem::new(
                "open_directory".to_string(),
                "Open Directory",
            ))
            .add_item(CustomMenuItem::new("refresh".to_string(), "Refresh")),
    );
    let menu = Menu::new()
        .add_submenu(file_menu)
        .add_native_item(MenuItem::Separator);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "open_directory" => {
					let mut path: String = String::new();
                    dialog::FileDialogBuilder::new().pick_folder(|folder_path| {
						path = folder_path.unwrap().to_string_lossy().to_string();
                        // do something with the optional folder path here
                        // the folder path is `None` if the user closed the dialog
                        // get_files(folder_path.unwrap());
                    });
					event
							.window()
							.emit(
								"open_directory",
								Payload {
									message: path,
								},
							)
							.unwrap();
                }
                "refresh" => {
                    /*emit event for frontend here*/
                    event
                        .window()
                        .emit(
                            "refresh",
                            Payload {
                                message: "hello world".to_string(),
                            },
                        )
                        .unwrap();
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![greet])
        // .invoke_handler(tauri::generate_handler![getFiles])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
